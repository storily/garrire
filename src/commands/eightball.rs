use crate::Locale;
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

group!({
    name: "eightball",
    options: {
        prefix: "8ball",
        default_command: eightball
    },
    commands: [eightball],
});

#[command]
fn eightball(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, Locale::new(&["8ball"]).get("eight-ball", None))?;
    Ok(())
}
