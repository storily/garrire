use crate::{locale_args, Locale};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

group!({
    name: "ping",
    options: {},
    commands: [ping],
});

#[command]
fn ping(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(
        &ctx.http,
        Locale::new(&["main"]).get(
            "pong",
            Some(&locale_args! {
                "message" => args.message()
            }),
        ),
    )?;
    Ok(())
}
