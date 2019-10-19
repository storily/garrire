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
    name: "colour",
    options: {},
    commands: [colour],
});

#[command]
fn colour(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    get_help!("colour", ctx, msg, args);

    let main = Locale::new(&["main"]).unwrap();
    let amount: usize = args.single().unwrap_or(1);

    let pick: Vec<String> = (0..amount)
        .map(|_| main.get("colours", None).unwrap())
        .collect();

    msg.channel_id.say(&ctx.http, main.list(pick).unwrap())?;
    Ok(())
}
