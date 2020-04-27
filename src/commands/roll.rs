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

#[allow(unused_parens)]
mod parser;

#[derive(Debug)]
pub struct Roll {
    pub n: f64,
    pub sides: usize,
    pub offset: f64,
}

impl Roll {
    pub fn roll(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();

        let whole = self.n.floor();
        let rest = self.n - whole;

        let mut rolls = vec![1_f64; whole as _];
        if rest > 0_f64 {
            rolls.push(rest);
        }

        rolls
            .into_iter()
            .map(|mult| {
                mult * if self.sides <= 1 {
                    1_f64
                } else {
                    rng.gen_range(1, self.sides) as _
                } + self.offset
            })
            .collect()
    }
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = String::from(if self.offset < 0_f64 { "" } else { "+" }) + &self.offset.to_string();

        write!(
            f,
            "{}d{}{}",
            if self.n == 0_f64 {
                String::from("")
            } else {
                self.n.to_string()
            },
            self.sides,
            if self.offset == 0_f64 { "" } else { &o }
        )
    }
}

#[group]
#[commands(roll)]
pub struct ROLL;

#[command]
fn roll(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    get_help!(on_empty, "roll", ctx, msg, args);

    let die = parser::DieParser::new();

    let (rolls, dice): (Vec<Vec<f64>>, Vec<String>) = args
        .raw_quoted()
        .filter_map(|arg| die.parse(arg).ok())
        .map(|d| (d.roll(), d.to_string()))
        .unzip();

    let rolls: Vec<f64> = rolls.into_iter().flatten().collect();

    if rolls.is_empty() {
        return super::help(ctx, msg, "roll");
    }

    let sum: f64 = rolls.iter().fold(0_f64, |sum, i| sum + i);

    let head = format!(
        "{} ⇒ **{}**",
        dice.join(" "),
        rolls
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    msg.channel_id.say(
        &ctx.http,
        if rolls.len() > 1 {
            format!("{} = {}", head, sum)
        } else {
            head
        },
    )?;

    Ok(())
}
