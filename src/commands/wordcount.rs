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

        let list = if let Ok(second) = args.single::<String>() {
            second == "list"
        } else {
            false
        };

        let reply = if list {
            match get_wordcount_list(&username) {
                Ok(counts) => nanos
                    .get(
                        "count-list",
                        Some(&locale_args! {
                            prefix,
                            "username" => username,
                            "counts" => counts
                        }),
                    )
                    .unwrap(),
                Err(err) => do_error(err, username),
            }
        } else {
            match get_wordcount(&username) {
                Ok(count) => nanos
                    .get(
                        "count",
                        Some(&locale_args! {
                            prefix,
                            "username" => username,
                            "count" => count
                        }),
                    )
                    .unwrap(),
                Err(err) => do_error(err, username),
            }
        };

        fn do_error(err: impl std::error::Error, username: String) -> String {
            log::warn!("wordcount fetch error: {}\n{:?}", err, err);
            let nanos = Locale::new(&["nano"]).unwrap();

            nanos
                .get(
                    "error",
                    Some(&locale_args! {
                        prefix,
                        "username" => username
                    }),
                )
                .unwrap()
        }

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

fn get_wordcount_list(username: &str) -> Result<String> {
    let nano = nanowrimo::Nano::load().ok_or_else(|| unreachable_err())?;
    Ok(format!("{:?}", nano.wordcounts(username)?))
}
