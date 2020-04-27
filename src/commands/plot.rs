use crate::{get_help, Locale};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

#[group]
#[commands(plot)]
pub struct PLOT;

#[command]
fn plot(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!("plot", ctx, msg, args);
    msg.channel_id.say(
        &ctx.http,
        Locale::single("plots", "plot", None, None).unwrap(),
    )?;
    Ok(())
}
