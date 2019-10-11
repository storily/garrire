use diesel::r2d2::{ConnectionManager, ManageConnection, Pool};
use diesel::PgConnection;
use log::{error, info};
use serde::Deserialize;
use serenity::client::Client;
use serenity::framework::StandardFramework;
use serenity::model::gateway::Activity as DiscAct;
use std::collections::HashMap;
use std::sync::Arc;

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
pub struct Locale {
    /// Locale to use as the default
    #[serde(default = "Locale::default_lang")]
    pub default: String,

    /// Default glitchiness
    ///
    /// By default garrīre has a locale glitch 1% of the time. A locale glitch
    /// is when the bot answers in a different language than expected, which is
    /// pulled from the "rest of available locales" than requested.
    ///
    /// With this setting, the frequency can be adjusted, or glitchiness can be
    /// turned off completely (value at or below zero). A value of 1.0 or more
    /// is 100% locale glitchiness.
    #[serde(default = "Locale::default_glitch")]
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
    #[serde(default = "HashMap::new")]
    pub fallbacks: HashMap<String, Vec<String>>,
}

impl Locale {
    fn default_lang() -> String {
        "en-NZ".into()
    }

    const fn default_glitch() -> f64 {
        0.01
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            default: Self::default_lang(),
            glitchiness: Self::default_glitch(),
            fallbacks: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Activity {
    Playing(String),
    Listening(String),
    Streaming(String),
    Random,
    Leave,
}

impl Default for Activity {
    fn default() -> Self {
        Activity::Random
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
    pub locale: Locale,

    pub database: Database,
    pub discord: Discord,
}

impl Settings {
    pub fn load() -> Arc<Self> {
        lazy_static::lazy_static! {
            pub static ref SETTINGS: Arc<Settings> = {
                let mut settings = config::Config::default();
                settings
                .merge(config::File::with_name("Settings"))
                .unwrap()
                .merge(config::Environment::with_prefix("GARRIRE"))
                .unwrap();

                Arc::new(settings.try_into::<Settings>().unwrap())
            };
        }

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
            Activity::Random => {
                use rand::Rng;
                let (name, converter): (&str, fn(String) -> Activity) =
                    match rand::thread_rng().gen_range(0, 2) {
                        0 => ("now-playing", |s| Activity::Playing(s)),
                        1 => ("now-listening", |s| Activity::Listening(s)),
                        2 => ("now-streaming", |s| Activity::Streaming(s)),
                        _ => unreachable!(),
                    };

                let active = crate::Locale::glitchy(&["now-what"]).random(name, None);
                self.discord_activity(&converter(active))
            }
        }
    }

    pub fn new_activity(&self) -> Option<DiscAct> {
        self.discord_activity(&self.activity)
    }

    fn default_streaming_url() -> String {
        env!("CARGO_PKG_HOMEPAGE").into()
    }
}
