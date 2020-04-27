#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate diesel;

use log::{debug, error, info};
use std::sync::Arc;

pub(crate) use locale::Locale;
pub(crate) use settings::{DbPool, Settings};

#[macro_use]
mod commands;
mod error;
mod handler;
#[macro_use]
mod locale;
mod nanowrimo;
mod palindromic;
mod schema;
mod settings;
mod voice;

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

    info!("Vocalising...");
    {
        let mut data = client.data.write();
        data.insert::<voice::Manager>(Arc::clone(&client.voice_manager));
    }

    info!("Selecting commands...");
    client.with_framework(settings.discord.framework());

    info!("Starting up...");
    client.start()?;

    Ok(())
}
