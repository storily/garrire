use crate::get_help;
use rand::Rng;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(pick)]
pub struct PICK;

#[command]
fn pick(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    get_help!(on_empty, "pick", ctx, msg, args);

    if let Ok(start) = args.single::<isize>() {
        if let Ok(end) = args.single::<isize>() {
            let range = &mut [start, end];
            range.sort();
            let [start, end] = range;

            msg.channel_id.say(
                &ctx.http,
                if start == end {
                    *start
                } else {
                    rand::thread_rng().gen_range(*start, *end)
                },
            )?;
            return Ok(());
        }
    }

    super::help(ctx, msg, "pick")
}
