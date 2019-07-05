use diesel::r2d2::{ConnectionManager, ManageConnection, Pool};
use diesel::PgConnection;
use fluent::FluentBundle;
use fluent_resmgr::resource_manager::ResourceManager;
use log::{error, info};
use serde::Deserialize;
use serenity::client::Client;
use serenity::framework::StandardFramework;
use std::path::PathBuf;

use crate::handler::Handler;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

impl Database {
    pub fn connect(&self) -> Pool<ConnectionManager<PgConnection>> {
        let manager = ConnectionManager::<PgConnection>::new(self.url.clone());
        Pool::new(manager).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Discord {
    pub token: String,

    #[serde(default = "Discord::default_prefix")]
    pub prefix: String,

    #[serde(default)]
    pub commands: Vec<String>,
}

impl Discord {
    pub fn client<M>(&self, db: Pool<M>) -> Client
    where
        M: ManageConnection,
    {
        Client::new(&self.token, Handler { db }).unwrap()
    }

    pub fn framework(&self) -> StandardFramework {
        let mut fw = StandardFramework::new().configure(|c| c.prefix(&self.prefix));

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

#[derive(Debug, Deserialize)]
pub struct Language {
    #[serde(default = "Language::default_path")]
    pub path: PathBuf,

    /// Locale to use as the default
    #[serde(default = "Language::default_lang")]
    pub default: String,

    /// Locales to use as alternates
    #[serde(default = "Vec::new")]
    pub alternates: Vec<String>,
}

impl Language {
    fn default_path() -> PathBuf {
        if cfg!(debug_assertions) {
            "./lang"
        } else {
            "/usr/share/garrire/lang"
        }
        .into()
    }

    fn default_lang() -> String {
        "en-NZ".into()
    }

    pub fn resource_manager(&self) -> ResourceManager {
        ResourceManager::new(
            self.path
                .join("{locale}/{res_id}")
                .to_string_lossy()
                .to_string(),
        )
    }

    pub fn resources(&self) -> Vec<String> {
        use std::collections::HashSet;

        fn is_hidden(entry: &walkdir::DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }

        walkdir::WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok().map(|f| f.file_name().to_os_string()))
            .filter_map(|f| {
                PathBuf::from(&f)
                    .extension()
                    .map(|s| s == "ftl")
                    .and(f.into_string().ok())
            })
            .collect::<HashSet<String>>()
            .into_iter()
            .collect()
    }

    pub fn bundle<'r>(&self, mgr: &'r ResourceManager) -> FluentBundle<'r> {
        self.bundle_for(mgr, &self.default)
    }

    pub fn bundle_for<'r>(&self, mgr: &'r ResourceManager, lang: &str) -> FluentBundle<'r> {
        let mut locales = self.alternates.clone();
        locales.push(self.default.clone());
        dbg!(&locales);

        mgr.get_bundle(locales, self.resources())
    }
}

impl Default for Language {
    fn default() -> Self {
        Self {
            path: Self::default_path(),
            default: "en-NZ".into(),
            alternates: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub language: Language,

    pub database: Database,
    pub discord: Discord,
}

impl Settings {
    pub fn load() -> Self {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name("Settings"))
            .unwrap()
            .merge(config::Environment::with_prefix("GARRIRE"))
            .unwrap();
        settings.try_into::<Settings>().unwrap()
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
}
