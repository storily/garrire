use serenity::{
    client::Context,
    framework::standard::{macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;
use crate::Settings;

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
        Locale::single("help", topic, Some(&locale_args! { prefix }), None)
            .unwrap_or("No help available :(".into()),
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

#[help]
fn top_level_help(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    use crate::{locale_args, Locale};
    use rand::Rng;

    let helps = Locale::new(&["help"]).unwrap();
    let prefix = &Settings::load().discord.prefix;
    let mut gs: Vec<String> = groups.iter()
        .map(|g| g.options.prefixes.first().map(|s| *s).unwrap_or(g.name))
        .chain(std::iter::once("help"))
        .map(|g| format!("`{}{}`", prefix, g))
        .collect();
    gs.sort();

    dbg!(help_options, groups, owners);

    msg.channel_id.say(
        &ctx.http,
        helps.get("help", Some(&locale_args! {
            prefix,
            "commandCount" => gs.len(),
            "commandList" => helps.list(gs).unwrap()
        })).unwrap(),
    )?;

    if rand::thread_rng().gen_range(0, 10) < 2 {
        msg.channel_id.say(
            &ctx.http,
            helps.get("help-syntax", None).unwrap(),
        )?;
    }

    Ok(())
}
