use diesel::r2d2::{ManageConnection, Pool};
use log::info;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::Settings;

pub struct Database<M> {
    marker: std::marker::PhantomData<M>
}
impl<M: ManageConnection> typemap::Key for Database<M> {
    type Value = Pool<M>;
}

pub struct Handler<M>
where
    M: ManageConnection,
{
    pub db: Pool<M>,
}

impl<M> EventHandler for Handler<M>
where
    M: ManageConnection,
{
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
