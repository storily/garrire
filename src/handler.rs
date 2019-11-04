use log::info;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::{DbPool, Settings};

pub struct Database;
impl typemap::Key for Database {
    type Value = DbPool;
}

pub struct Handler {
    pub db: DbPool,
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
