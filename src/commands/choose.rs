use crate::{get_help, Locale};
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
    get_help!("choose", ctx, msg, args);
    use rand::{seq::SliceRandom, Rng};

    let mut choices = Vec::new();
    let mut words = args.raw_quoted();
    let mut xormode = false;
    loop {
        let choice = (&mut words)
            .take_while(|item| match *item {
                "or" => false,
                "xor" => {
                    xormode = true;
                    false
                }
                _ => true,
            })
            .collect::<Vec<&str>>()
            .join(" ");

        if choice.is_empty() {
            break;
        } else {
            choices.push(choice);
        }
    }

    if choices.len() < 2 {
        super::help(ctx, msg, "choose")
    } else {
        let mut rng = rand::thread_rng();

        msg.channel_id.say(
            &ctx.http,
            if !xormode && rng.gen::<u8>() > 254 {
                [
                    "yes",
                    "both",
                    "all of the above",
                    "not super sure, actually",
                    "Gryffindor!",
                ]
                .choose(&mut rng)
                .unwrap()
            } else {
                choices.choose(&mut rng).unwrap().clone()
            },
        )?;
        Ok(())
    }
}
