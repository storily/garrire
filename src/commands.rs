use serenity::framework::standard::CommandGroup;

pub mod eightball;
pub mod ping;

pub use eightball::EIGHTBALL_GROUP;
pub use ping::PING_GROUP;

// These two arrays MUST be in sync, with each command group
// at the same index as its name.

pub static GROUPS: &'static [&'static CommandGroup] = &[&EIGHTBALL_GROUP, &PING_GROUP];

pub static NAMES: &'static [&'static str] = &["8ball", "ping"];
