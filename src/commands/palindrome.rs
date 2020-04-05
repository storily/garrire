use crate::{get_help, Locale};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

group!({
    name: "palindrome",
    commands: [palindrome],
});

#[command]
fn palindrome(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use crate::palindromic::next_palindromic;
    get_help!(on_empty, "palindrome", ctx, msg, args);

    if let Ok(n) = args.single::<usize>() {
        let np = next_palindromic(n);
        msg.channel_id
            .say(&ctx.http, format!("{} ({} + {})", np, n, np - n))?;
        Ok(())
    } else {
        super::help(ctx, msg, "palindrome")
    }
}
