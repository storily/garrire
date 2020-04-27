use crate::{get_help, Locale};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

#[group]
#[prefix = "8ball"]
#[commands(eightball)]
#[default_command(eightball)]
struct EIGHTBALL;

#[command]
fn eightball(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!("eightball", ctx, msg, args);
    msg.channel_id.say(
        &ctx.http,
        Locale::single("8ball", "eight-ball", None, None).unwrap(),
    )?;
    Ok(())
}
