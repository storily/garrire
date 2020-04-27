use crate::get_help;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(calc)]
struct CALC;

#[cfg(feature = "calc-rhai")]
#[command]
fn calc(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!(on_empty, "calc", ctx, msg, args);

    let expr = args.rest().trim();

    if expr.is_empty() {
        super::help(ctx, msg, "calc")
    } else {
        use rhai::{Engine, RegisterFn};
        let mut engine = Engine::new();
        engine.register_fn("+", |a: i64, b: f64| -> f64 { (a as f64) + b });
        engine.register_fn("+", |a: f64, b: i64| -> f64 { a + (b as f64) });
        engine.register_fn("-", |a: i64, b: f64| -> f64 { (a as f64) - b });
        engine.register_fn("-", |a: f64, b: i64| -> f64 { a - (b as f64) });
        engine.register_fn("*", |a: i64, b: f64| -> f64 { (a as f64) * b });
        engine.register_fn("*", |a: f64, b: i64| -> f64 { a * (b as f64) });
        engine.register_fn("/", |a: i64, b: f64| -> f64 { (a as f64) / b });
        engine.register_fn("/", |a: f64, b: i64| -> f64 { a / (b as f64) });

        msg.channel_id.say(
            &ctx.http,
            match engine.eval::<f64>(expr) {
                Ok(value) => format!("{} = {}", expr, value),
                Err(_) => match engine.eval::<i64>(expr) {
                    Ok(value) => format!("{} = {}", expr, value),
                    Err(err) => format!("{} = {}", expr, err),
                },
            },
        )?;
        Ok(())
    }
}

#[cfg(not(feature = "calc-rhai"))]
#[command]
fn calc(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!("calc", ctx, msg, args);
    msg.channel_id.say(&ctx.http, "Calculator not available!")?;
    Ok(())
}
