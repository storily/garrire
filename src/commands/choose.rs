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
    name: "choose",
    options: {},
    commands: [choose],
});

#[command]
fn choose(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!(on_empty, "choose", ctx, msg, args);
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

    let mut rng = rand::thread_rng();

    msg.channel_id.say(
        &ctx.http,
        match choices.as_slice() {
            &[] => return super::help(ctx, msg, "choose"),
            &[_] => {
                if rng.gen::<bool>() {
                    Locale::single("choose", "yes", None, None).unwrap_or("yes".into())
                } else {
                    Locale::single("choose", "no", None, None).unwrap_or("no".into())
                }
            }
            _ => {
                if !xormode && rng.gen::<u16>() == 1 {
                    Locale::single("choose", "both", None, None).unwrap_or("both".into())
                } else {
                    choices.choose(&mut rng).unwrap().clone()
                }
            }
        },
    )?;

    Ok(())
}
