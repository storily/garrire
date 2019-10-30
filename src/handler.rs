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
        let member = msg.member(ctx.cache);
        info!(
            "From: {} Message: {}",
            member
                .map(|m| m.display_name().into_owned())
                .unwrap_or("?unknown?".into()),
            msg.content
        );
    }
}
