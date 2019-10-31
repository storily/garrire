use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use log::{error, info};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serenity::client::Client;
use serenity::framework::StandardFramework;
use serenity::model::gateway::Activity as DiscAct;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

use crate::handler::{self, Handler};
use crate::nanowrimo::Nanowrimo;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

impl Database {
    pub fn connect(&self) -> crate::error::Result<DbPool> {
        let manager = ConnectionManager::<PgConnection>::new(self.url.clone());
        Ok(Pool::new(manager)?)
    }

    // pub fn from_context(ctx: &serenity::client::Context) -> DbPool {
    //     let data = ctx.data.read();
    //     // unwrap is safe as the database is always added to the context data
    //     data.get::<handler::Database>().unwrap().clone()
    // }
}

#[derive(Debug, Deserialize)]
pub struct Discord {
    /// The token of the discord bot. Mandatory.
    pub token: String,

    /// Command prefix. Defaults to `!`.
    #[serde(default = "Discord::default_prefix")]
    pub prefix: String,

    /// A list of discord user IDs who are exempt from permission checks.
    #[serde(default)]
    pub owners: HashSet<u64>,

    /// Enabled commands.
    #[serde(default)]
    pub commands: Vec<String>,
}

impl Discord {
    pub fn client(&self, db: DbPool) -> Client {
        let client = Client::new(&self.token, Handler { db: db.clone() }).unwrap();

        {
            let mut data = client.data.write();
            data.insert::<handler::Database>(db);
        }

        client
    }

    pub fn framework(&self) -> StandardFramework {
        let mut fw = StandardFramework::new()
            .configure(|c| {
                c.delimiters(vec![
                    "\u{0009}", "\u{0020}", "\u{00A0}", "\u{1680}", "\u{2000}", "\u{2001}",
                    "\u{2002}", "\u{2003}", "\u{2004}", "\u{2005}", "\u{2006}", "\u{2007}",
                    "\u{2008}", "\u{2009}", "\u{200A}", "\u{202F}", "\u{205F}", "\u{3000}",
                ])
                .prefix(&self.prefix)
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .owners(self.owners.iter().map(|id| (*id).into()).collect())
            })
            .help(&crate::commands::TOP_LEVEL_HELP);

        if self.commands.is_empty() {
            info!("Loading all commands: {:?}", crate::commands::NAMES);
            for group in crate::commands::GROUPS {
                fw = fw.group(group);
            }
        } else {
            for name in &self.commands {
                if let Some(i) = crate::commands::NAMES.iter().position(|n| n == name) {
                    if let Some(g) = crate::commands::GROUPS.get(i) {
                        info!("Loading {} command", name);
                        fw = fw.group(g);
                    } else {
                        panic!(
                            "{} definition is out of bounds! This is a bug, report it.",
                            name
                        );
                    }
                } else {
                    error!("{} is not a valid command name", name);
                }
            }
        }

        fw
    }

    fn default_prefix() -> String {
        "!".into()
    }
}

#[derive(Debug)]
pub struct Locale {
    /// Locale to use as the default
    pub default: LanguageIdentifier,

    /// Default glitchiness
    ///
    /// By default garrīre has a locale glitch 1% of the time. A locale glitch
    /// is when the bot answers in a different language than expected, which is
    /// pulled from the "rest of available locales" than requested.
    ///
    /// With this setting, the frequency can be adjusted, or glitchiness can be
    /// turned off completely (value at or below zero). A value of 1.0 or more
    /// is 100% locale glitchiness.
    pub glitchiness: f64,

    /// Custom fallback chains
    ///
    /// By default locales fall back to the default if a resource or item is
    /// missing. In this setting, more advanced fallback chains can be defined.
    /// Fallback chains also affect date/time formatting.
    ///
    /// # Examples
    ///
    /// ```text
    /// [locale.fallbacks]
    /// "en-PIRATE": ["en-MIDDLE", "en-UK", "en-NZ"]
    /// "pt-BR": ["pt-PT"]
    /// "mi-NZ": []
    /// ```
    ///
    /// Here, Pirate English falls back to Middle English first, then to UK
    /// English, and finally to New Zealand English. Brazillian Portuguese only
    /// falls back to (European) Portuguese. And Te Reo Māori has no fallback.
    pub fallbacks: HashMap<LanguageIdentifier, Vec<LanguageIdentifier>>,
}

#[derive(Debug, Deserialize)]
#[doc(hidden)]
pub struct LocaleParsed {
    #[serde(default = "LocaleParsed::default_lang")]
    pub default: String,

    #[serde(default = "LocaleParsed::default_glitch")]
    pub glitchiness: f64,

    #[serde(default = "HashMap::new")]
    pub fallbacks: HashMap<String, Vec<String>>,
}

impl LocaleParsed {
    fn default_lang() -> String {
        "en-NZ".into()
    }

    const fn default_glitch() -> f64 {
        0.01
    }
}

impl Default for LocaleParsed {
    fn default() -> Self {
        Self {
            default: Self::default_lang(),
            glitchiness: Self::default_glitch(),
            fallbacks: HashMap::new(),
        }
    }
}

impl From<LocaleParsed> for Locale {
    fn from(parsed: LocaleParsed) -> Self {
        Self {
            default: parsed.default.parse().unwrap(),
            glitchiness: parsed.glitchiness,
            fallbacks: parsed
                .fallbacks
                .into_iter()
                .map(|(key, val)| {
                    (
                        key.parse().unwrap(),
                        val.into_iter().map(|lang| lang.parse().unwrap()).collect(),
                    )
                })
                .collect(),
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        LocaleParsed::default().into()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Activity {
    Playing(String),
    Listening(String),
    Streaming(String),
    Locale,
    Leave,
}

impl Default for Activity {
    fn default() -> Self {
        Activity::Locale
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub activity: Activity,

    #[serde(default = "Settings::default_streaming_url")]
    pub streaming_url: String,

    #[serde(default)]
    #[serde(rename = "locale")]
    locale_parsed: LocaleParsed,

    #[serde(skip)]
    pub locale: Locale,

    pub database: Database,
    pub discord: Discord,
    pub nanowrimo: Option<Nanowrimo>,
}

impl Settings {
    pub fn load() -> Arc<Self> {
        // TODO: split into init->Result and load->Arc
        static SETTINGS: Lazy<Arc<Settings>> = Lazy::new(|| {
            let mut settings = config::Config::default();
            settings
                .merge(config::File::with_name("Settings"))
                .unwrap()
                .merge(config::Environment::with_prefix("GARRĪRE"))
                .unwrap()
                .merge(config::Environment::with_prefix("GARRIRE"))
                .unwrap();

            let mut settings: Settings = settings.try_into().unwrap();
            let locale_parsed =
                std::mem::replace(&mut settings.locale_parsed, LocaleParsed::default());
            settings.locale = locale_parsed.into();
            Arc::new(settings)
        });

        SETTINGS.clone()
    }

    pub fn logging(&self) {
        let mut logs = env_logger::Builder::from_default_env();
        logs.filter(
            Some(env!("CARGO_PKG_NAME")),
            if self.debug {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            },
        );
        logs.init();
    }

    fn discord_activity(&self, this_activity: &Activity) -> Option<DiscAct> {
        match this_activity {
            Activity::Leave => None,
            Activity::Playing(s) => Some(DiscAct::playing(&s)),
            Activity::Listening(s) => Some(DiscAct::listening(&s)),
            Activity::Streaming(s) => Some(DiscAct::streaming(&s, &self.streaming_url)),
            Activity::Locale => {
                let active = crate::Locale::glitchy(&["now-what"])
                    .and_then(|l| l.get("now-what", None))
                    .expect("Could not get now-what value out of locale");

                let listening = "Listening to ";
                let playing = "Playing ";
                let streaming = "Streaming ";

                self.discord_activity(&if active.starts_with(listening) {
                    Activity::Listening(active[listening.len()..].into())
                } else if active.starts_with(playing) {
                    Activity::Playing(active[playing.len()..].into())
                } else if active.starts_with(streaming) {
                    Activity::Streaming(active[streaming.len()..].into())
                } else {
                    unreachable!()
                })
            }
        }
    }

    pub fn new_activity(&self) -> Option<DiscAct> {
        self.discord_activity(&self.activity)
    }

    fn default_streaming_url() -> String {
        env!("CARGO_PKG_HOMEPAGE").into()
    }

    pub fn nano(&self) -> Option<&Nanowrimo> {
        self.nanowrimo.as_ref()
    }
}
