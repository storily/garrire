use crate::{locale_args, Locale, Settings};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

group!({
    name: "choose",
    options: {},
    commands: [choose],
});

#[command]
fn choose(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use rand::seq::SliceRandom;

    let mut choices = Vec::new();
    let mut words = args.raw_quoted();
    loop {
        let choice = (&mut words)
            .take_while(|item| match *item {
                "or" | "xor" => false,
                _ => true
            })
            .collect::<Vec<&str>>()
            .join(" ");

        if choice.is_empty() {
            break;
        } else {
            choices.push(choice);
        }
    }

    let help = Locale::new(&["help"]).get(
        "choose",
        Some(&locale_args! {
            "prefix" => Settings::load().discord.prefix.clone()
        }),
    );

    msg.channel_id.say(
        &ctx.http,
        choices.choose(&mut rand::thread_rng()).unwrap_or(&help),
    )?;
    Ok(())
}
