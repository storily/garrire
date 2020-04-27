use log::info;
use serenity::{
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};

use crate::{voice, DbPool, Settings};

pub struct Database;
impl typemap::Key for Database {
    type Value = DbPool;
}

pub struct Handler {
    pub db: DbPool,
}

macro_rules! regex {
    ($name:ident, $re:literal $(,)?) => {{
        static $name: ::once_cell::sync::OnceCell<::regex::Regex> =
            ::once_cell::sync::OnceCell::new();
        $name.get_or_init(|| ::regex::Regex::new($re).unwrap())
    }};
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        if let Some(act) = Settings::load().new_activity() {
            ctx.set_activity(act);
        }

        info!("Ready and able!\n");
    }

    fn message(&self, ctx: Context, msg: Message) {
        use serenity::model::channel::Channel;

        let member = msg.member(&ctx.cache);
        let channel = msg.channel(&ctx.cache);

        if let (Some(guild), Ok(current), Some(member)) = (
            &msg.guild(&ctx.cache),
            &ctx.http.get_current_user(),
            &member,
        ) {
            if Settings::load().debug || current.id == member.user_id() {
                if regex!(RE_NEAR, r"Wordwar \d+ is starting in 30 seconds!").is_match(&msg.content)
                {
                    voice::ding(voice::Ding::WordwarNear, &ctx, guild.clone());
                } else if regex!(RE_START, r"Wordwar \d+ is starting now!").is_match(&msg.content) {
                    voice::ding(voice::Ding::WordwarStart, &ctx, guild.clone());
                } else if regex!(RE_END, r"Wordwar \d+ has ended!").is_match(&msg.content) {
                    voice::ding(voice::Ding::WordwarEnd, &ctx, guild.clone());
                }
            }
        }

        info!(
            "[{}] [{}] {}",
            channel
                .map(|c| match c {
                    Channel::Guild(gc) => {
                        let chan = "#".to_string() + &gc.read().name;
                        gc.read()
                            .category_id
                            .map(|cat| {
                                let cat = cat.to_channel(ctx.http);
                                cat.map(|cat| cat.category().unwrap().read().name.clone())
                                    .unwrap_or("?category?".into())
                                    + "/"
                                    + &chan
                            })
                            .unwrap_or(chan)
                    }
                    c => c.to_string(),
                })
                .unwrap_or("#unknown#".into()),
            member
                .map(|m| m.display_name().into_owned())
                .unwrap_or("?unknown?".into()),
            msg.content
        );
    }
}
