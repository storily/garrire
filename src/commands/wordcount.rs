use crate::{error::*, get_help, nanowrimo, settings::Database};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(wc)]
pub struct WC;

#[derive(Clone, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: i64,
    pub nick: Option<String>,
    pub nano_user: Option<String>,
    pub tz: String,
}

#[command]
#[aliases("count", "wordcount", "!")]
fn wc(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use crate::{locale_args, Locale};
    get_help!("wordcount", ctx, msg, args);

    let db = Database::from_context(ctx);
    let dbuser: Option<User> = {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        use std::convert::TryInto;

        let author_id: i64 = msg.author.id.0.try_into().unwrap();
        users
            .select((id, discord_id, nick, nano_user, tz))
            .filter(discord_id.eq(author_id))
            .first(&db.get().unwrap())
            .optional()
            .unwrap()
    };

    let mut list = false;
    match args.rest() {
        "" => {}
        "list" => {
            list = true;
        }
        _ => return super::help(ctx, msg, "wordcount"),
    }

    let nanos = Locale::new(&["nano"]).unwrap();

    let reply = match dbuser.and_then(|u| u.nano_user) {
        None => nanos
            .get("no-nano-user", Some(&locale_args! { prefix }))
            .unwrap(),
        Some(username) => {
            if let Some(chan) = msg.channel(&ctx.cache) {
                if let Some(guild) = chan.guild() {
                    if let Some(lock) = guild.try_read() {
                        lock.broadcast_typing(&ctx.http).ok();
                    }
                }
            }

            if list {
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
                                "count" => count,
                                "palindromic" => crate::palindromic::is_palindromic(count) as u8
                            }),
                        )
                        .unwrap(),
                    Err(err) => do_error(err, username),
                }
            }
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
                    "username" => username,
                    "detail" => err.to_string()
                }),
            )
            .unwrap()
    }

    msg.channel_id.say(&ctx.http, reply)?;
    return Ok(());
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
