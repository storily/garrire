#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate diesel;

pub(crate) use locale::Locale;
use log::{debug, error, info};
pub(crate) use settings::{DbPool, Settings};

#[macro_use]
mod commands;
mod error;
mod handler;
mod nanowrimo;
#[macro_use]
mod locale;
mod schema;
mod settings;

fn main() {
    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Wrapping main...");

    if let Err(err) = start() {
        error!("{}", err);
        debug!("{:?}", err);
    } else {
        info!("Exiting gracefully");
    }
}

fn start() -> error::Result<()> {
    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Reading configuration...");
    let settings = Settings::load();

    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Hooking up logs...");
    settings.logging();

    info!("Connecting to database...");
    let pool = settings.database.connect()?;

    if settings.nanowrimo.is_some() {
        info!("Preparing nanowrimo client...");
        nanowrimo::Nano::init(pool.clone())?;
    }

    info!("Preparing discord client...");
    let mut client = settings.discord.client(pool.clone());

    info!("Selecting commands...");
    client.with_framework(settings.discord.framework());

    info!("Starting up...");
    client.start()?;

    Ok(())
}
