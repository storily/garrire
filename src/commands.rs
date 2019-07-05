use serenity::framework::standard::CommandGroup;

pub mod ping;

pub use ping::PING_GROUP;

// These two arrays MUST be in sync, with each command group
// at the same index as its name.

pub static GROUPS: &'static [&'static CommandGroup] = &[&PING_GROUP];

pub static NAMES: &'static [&'static str] = &["ping"];
