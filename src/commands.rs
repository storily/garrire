use crate::Settings;
use serenity::{
    client::Context,
    framework::standard::{macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

pub mod calc;
pub mod choose;
pub mod colour;
pub mod eightball;
pub mod motivate;
pub mod pick;
pub mod ping;
pub mod plot;
pub mod roll;
pub mod wordcount;

// These two arrays MUST be in sync, with each command group
// at the same index as its name.

pub static GROUPS: &'static [&'static CommandGroup] = &[
    &eightball::EIGHTBALL_GROUP,
    &calc::CALC_GROUP,
    &choose::CHOOSE_GROUP,
    &colour::COLOUR_GROUP,
    &motivate::MOTIVATE_GROUP,
    &pick::PICK_GROUP,
    &ping::PING_GROUP,
    &plot::PLOT_GROUP,
    &roll::ROLL_GROUP,
    &wordcount::WC_GROUP,
    // LATER: name, prompt
];

pub const NAMES: &'static [&'static str] = &[
    "8ball",
    "calc",
    "choose",
    "colour",
    "motivate",
    "pick",
    "ping",
    "plot",
    "roll",
    "wordcount",
];

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
    (on_empty, $topic:expr, $ctx:expr, $msg:expr, $args:expr) => {
        match $args.current() {
            None | Some("--help") | Some("help") | Some("-help") | Some("-h") | Some("?") => {
                return crate::commands::help($ctx, $msg, $topic)
            }
            _ => {}
        }
    };
    ($topic:expr, $ctx:expr, $msg:expr, $args:expr) => {
        match $args.current() {
            Some("--help") | Some("help") | Some("-help") | Some("-h") | Some("?") => {
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
    _args: Args,
    _help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    _owners: HashSet<UserId>,
) -> CommandResult {
    use crate::{locale_args, Locale};
    use rand::Rng;

    let helps = Locale::new(&["help"]).unwrap();
    let prefix = &Settings::load().discord.prefix;
    let mut gs: Vec<String> = groups
        .iter()
        .map(|g| g.options.prefixes.first().map(|s| *s).unwrap_or(g.name))
        .chain(std::iter::once("help"))
        .map(|g| format!("`{}{}`", prefix, g))
        .collect();
    gs.sort();

    msg.channel_id.say(
        &ctx.http,
        helps
            .get(
                "help",
                Some(&locale_args! {
                    prefix,
                    "commandCount" => gs.len(),
                    "commandList" => helps.list(gs).unwrap()
                }),
            )
            .unwrap(),
    )?;

    if rand::thread_rng().gen_range(0, 10) < 2 {
        msg.channel_id
            .say(&ctx.http, helps.get("help-syntax", None).unwrap())?;
    }

    Ok(())
}
