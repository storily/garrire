#![feature(async_await)]

use log::info;

mod commands;
mod handler;
mod settings;

#[runtime::main]
async fn main() {
    use settings::Settings;

    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Reading configuration...");
    let settings = Settings::load();

    #[cfg(debug_assertions)]
    eprintln!("(pre-logging) Hooking up logs...");
    settings.logging();

    info!("Loading language...");
    let langmgr = settings.language.resource_manager();
    let lang = settings.language.bundle(&langmgr);
    dbg!(lang.format("hello-world", None));
    //mgr.get_bundle((), ());

    info!("Connecting to database...");
    let pool = settings.database.connect();

    info!("Preparing discord client...");
    let mut client = settings.discord.client(pool.clone());

    info!("Selecting commands...");
    client.with_framework(settings.discord.framework());

    // info!("Starting up...");
    // if let Err(why) = client.start() {
    //     error!("Err with client: {:?}", why);
    // }
}
