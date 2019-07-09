use crate::{FluentValue, Locale};
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
        Locale::new(&["main"]).simple(
            "pong",
            Some(
                &vec![("message", FluentValue::from(args.message()))]
                    .iter()
                    .cloned()
                    .collect(),
            ),
        ),
    )?;
    Ok(())
}
