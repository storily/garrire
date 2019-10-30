use crate::error::*;
use crate::schema::nanowrimo_logins as auth_table;
use crate::{DbPool, Settings};
use error_chain::bail;
use isahc::prelude::ResponseExt;
use log::debug;
use models::*;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_json::{json, to_vec};
use std::borrow::Cow;
use std::sync::{Arc, RwLock};

pub mod models;

const SERVER: &str = "https://api.nanowrimo.org";
const SIGN_IN: &str = "/users/sign_in";
const REGION_ANNOUNCES: &str = "/groups";

fn url(routes: &[&str]) -> String {
    SERVER.to_string() + &routes.join("/")
}

pub struct Nano {
    auth: RwLock<Auth>,
    settings: Arc<Settings>,
    db: DbPool,
}

static INSTANCE: OnceCell<Option<Arc<Nano>>> = OnceCell::new();
impl Nano {
    fn token(&self) -> String {
        use std::borrow::Borrow;
        self.auth
            .read()
            .expect("poisoned lock in Nano::get")
            .token
            .clone()
    }

    fn get<T>(&self, url: String) -> Result<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        use isahc::prelude::{Request, RequestExt};
        let mut resp = Request::get(&url)
            .header("Authorization", self.token())
            .body(())?
            .send()?;
        if !resp.status().is_success() {
            let err: NanoError = resp.json()?;
            bail!(if err.error.starts_with("Not Authorized") {
                ErrorKind::NanoUnauthorised(url)
            } else {
                ErrorKind::Nano(resp.json()?)
            });
        }

        Ok(resp.json()?)
    }

    pub fn init(db: DbPool) -> Result<()> {
        if INSTANCE.get().is_some() {
            return Ok(());
        }

        let settings = Settings::load();
        let value = if let Some(ref auther) = settings.nano() {
            let auth = RwLock::new(auther.get_token(&db)?);
            Some(Arc::new(Self { auth, settings, db }))
        } else {
            None
        };

        INSTANCE
            .set(value)
            .map(|_| ())
            .map_err(|_| unreachable_err())
    }

    pub fn load() -> Option<Arc<Self>> {
        INSTANCE
            .get()
            .expect("nanowrimo was not initialised")
            .clone()
    }

    pub fn reauth(&self) -> Result<()> {
        self.settings
            .nano()
            .ok_or(unreachable_err())
            .and_then(|auther| {
                // TODO: figure out what order this runs in: we want to lock after get_token,
                // even if we end up doing useless work, so as to keep the reads going.
                *self.auth.write().expect("poisoned lock in Nano::reauth") =
                    auther.get_token(&self.db)?;
                Ok(())
            })
    }

    pub fn region(&self, name: &str) -> Result<Object<Group>> {
        let res: Response = self.get(url(&[REGION_ANNOUNCES, name]))?;
        if let Data::Group(object) = res.data {
            Ok(object)
        } else {
            Err(nano_unexpected::<Group>(res.data))
        }
    }
}

#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "auth_table"]
pub struct Auth {
    #[serde(skip)]
    pub token: String,

    #[serde(rename = "exp")]
    pub expiry: i32,

    pub user_id: i32,
}

impl Auth {
    pub fn parse(token: String) -> Result<Self> {
        let payload = token
            .split('.')
            .skip(1)
            .next()
            .ok_or(ErrorKind::NanoTokenFormat)?;
        let mut auth: Auth = serde_json::from_str(&String::from_utf8(base64::decode(payload)?)?)?;
        auth.token = token;
        Ok(auth)
    }
}

#[derive(Debug, Deserialize)]
pub struct Nanowrimo {
    /// Identifier for a nano user
    pub user: String,

    /// Password for the same
    pub password: String,
}

impl Nanowrimo {
    pub fn get_token(&self, db: &DbPool) -> Result<Auth> {
        debug!("checking database for valid auth");
        let mut auth: Option<Auth> = {
            use auth_table::dsl::*;
            use diesel::prelude::*;
            nanowrimo_logins
                .select((token, expiry, user_id))
                .filter(expiry.gt(diesel::dsl::sql("extract(epoch from now()) - 10")))
                .order_by(created_at.desc())
                .first(&db.get()?)
                .optional()?
        };

        if let Some(ref login) = auth {
            debug!("got still-valid auth: {:?}", login);
        } else {
            debug!("auth expired, performing login to nanowrimo");
            let login = self.login()?;
            debug!("got auth: {:?}, saving", login);

            {
                use diesel::prelude::*;
                diesel::insert_into(auth_table::table)
                    .values(&login)
                    .execute(&db.get()?)?;
            }

            auth = Some(login);
        }

        auth.ok_or(unreachable_err())
    }

    fn login(&self) -> Result<Auth> {
        use isahc::prelude::{Request, RequestExt};
        let mut resp = Request::post(url(&[SIGN_IN]))
            .header("Content-type", "application/json")
            .body(to_vec(&json!({
                "identifier": self.user,
                "password": self.password,
            }))?)?
            .send()?;

        if !resp.status().is_success() {
            bail!("generic nano fail");
        }

        let models::Login { auth_token } = resp.json()?;
        Auth::parse(auth_token)
    }
}
