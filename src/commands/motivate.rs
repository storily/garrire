use crate::{get_help, Locale};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

group!({
    name: "motivate",
    options: {},
    commands: [motivate],
});

#[command]
fn motivate(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!("motivate", ctx, msg, args);
    msg.channel_id.say(
        &ctx.http,
        Locale::single("main", "motivation", None, None).unwrap(),
    )?;
    Ok(())
}
