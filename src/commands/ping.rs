use crate::{get_help, locale_args, Locale};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

#[group]
#[commands(ping)]
pub struct PING;

#[command]
fn ping(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!("ping", ctx, msg, args);
    msg.channel_id.say(
        &ctx.http,
        Locale::single(
            "main",
            "pong",
            Some(&locale_args! {
                "message" => args.message()
            }),
            None,
        )
        .unwrap_or("pong".into()),
    )?;
    Ok(())
}
