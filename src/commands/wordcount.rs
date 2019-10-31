use crate::{error::*, get_help, nanowrimo};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

group!({
    name: "wc",
    options: {},
    commands: [wc],
});

#[command]
fn wc(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use crate::{locale_args, Locale};
    get_help!(on_empty, "wordcount", ctx, msg, args);

    if let Ok(username) = args.single::<String>() {
        let nanos = Locale::new(&["nano"]).unwrap();

        let reply = match get_wordcount(&username) {
            Ok(count) => nanos.get(
                "count",
                Some(&locale_args! {
                    prefix,
                    "username" => username,
                    "count" => count
                }),
            ),
            Err(err) => {
                log::warn!("wordcount fetch error: {}\n{:?}", err, err);
                nanos.get(
                    "error",
                    Some(&locale_args! {
                        prefix,
                        "username" => username
                    }),
                )
            }
        }
        .unwrap();

        msg.channel_id.say(&ctx.http, reply)?;
        return Ok(());
    }

    super::help(ctx, msg, "wordcount")
}

fn get_wordcount(username: &str) -> Result<usize> {
    let nano = nanowrimo::Nano::load().ok_or_else(|| unreachable_err())?;
    let current_event = nano
        .settings
        .current_event
        .clone()
        .unwrap_or("NaNoWriMo 2019".into());

    let counts = nano.wordcounts(username)?;
    let count = counts
        .iter()
        .find(|(k, _)| k == &&current_event)
        .map(|(_, v)| v)
        .unwrap_or(&0);

    Ok(*count)
}
