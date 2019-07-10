#[macro_use]
extern crate rust_embed;

pub(crate) use locale::Locale;
use log::{error, info};
pub(crate) use settings::Settings;

mod commands;
mod handler;
#[macro_use]
mod locale;
mod settings;

fn main() {
    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Reading configuration...");
    let settings = Settings::load();

    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Hooking up logs...");
    settings.logging();

    // dbg!(Locale::glitchy(&["main"]).random("now-playing", None));

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
