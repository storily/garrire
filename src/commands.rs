use serenity::{
    client::Context,
    framework::standard::{CommandGroup, CommandResult},
    model::channel::Message,
};

pub mod choose;
pub mod eightball;
pub mod ping;

pub use choose::CHOOSE_GROUP;
pub use eightball::EIGHTBALL_GROUP;
pub use ping::PING_GROUP;

// These two arrays MUST be in sync, with each command group
// at the same index as its name.

pub static GROUPS: &'static [&'static CommandGroup] =
    &[&EIGHTBALL_GROUP, &CHOOSE_GROUP, &PING_GROUP];

pub static NAMES: &'static [&'static str] = &["8ball", "choose", "ping"];

pub(crate) fn help(ctx: &mut Context, msg: &Message, topic: &str) -> CommandResult {
    use crate::{locale_args, Locale};
    msg.channel_id.say(
        &ctx.http,
        Locale::new(&["help"]).get(topic, Some(&locale_args! { prefix })),
    )?;
    Ok(())
}

#[macro_export]
macro_rules! get_help {
    ($topic:expr, $ctx:expr, $msg:expr, $args:expr) => {
        match $args.current() {
            None | Some("--help") | Some("help") | Some("-help") | Some("-h") | Some("?") => {
                return crate::commands::help($ctx, $msg, $topic)
            }
            _ => {}
        }
    };
}
