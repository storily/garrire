#[macro_use]
extern crate rust_embed;

pub(crate) use fluent::FluentValue;
pub(crate) use locale::Locale;
use log::{error, info};

mod commands;
mod handler;
mod locale;
mod settings;

fn main() {
    use settings::Settings;

    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Reading configuration...");
    let settings = Settings::load();

    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Hooking up logs...");
    settings.logging();

    info!("Loading language...");
    let loc = locale::Locale::new(&["main"]);
    dbg!(loc.simple("hello-world", None));

    info!("Connecting to database...");
    let pool = settings.database.connect();

    info!("Preparing discord client...");
    let mut client = settings.discord.client(pool.clone());

    info!("Selecting commands...");
    client.with_framework(settings.discord.framework());

    info!("Starting up...");
    if let Err(why) = client.start() {
        error!("Err with client: {:?}", why);
    }
}
