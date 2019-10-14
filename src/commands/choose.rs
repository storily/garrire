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

    let help = Locale::new(&["help"]).get("choose", Some(&locale_args! { prefix }));

    msg.channel_id.say(
        &ctx.http,
        if choices.is_empty() {
            &help
        } else {
            let mut rng = rand::thread_rng();

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
                choices.choose(&mut rng).unwrap().as_str()
            }
        },
    )?;
    Ok(())
}
