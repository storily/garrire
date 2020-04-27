use log::{error, info};
use serenity::{
    client::{bridge::voice::ClientVoiceManager, Context},
    model::{
        guild::Guild,
        misc::Mentionable,
        prelude::{ChannelId, GuildId},
    },
    prelude::{Mutex, RwLock, TypeMapKey},
    voice::ffmpeg,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Ding {
    WordwarNear,
    WordwarStart,
    WordwarEnd,
}

impl Ding {
    fn file(self) -> &'static str {
        match self {
            Self::WordwarNear => "voice/crotales.mp3",
            Self::WordwarStart => "voice/vibraphone.mp3",
            Self::WordwarEnd => "voice/glockenspiel.mp3",
        }
    }
}

pub(crate) struct Manager;

impl TypeMapKey for Manager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub(crate) fn ding(ding: Ding, ctx: &Context, guild: Arc<RwLock<Guild>>) {
    let guild_id = GuildId(630724997048041484);
    let connect_to = ChannelId(704259262783684668);

    let manager_lock = ctx
        .data
        .read()
        .get::<Manager>()
        .cloned()
        .expect("Expected voice::Manager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.join(guild_id, connect_to) {
        // handler.listen(Some(Box::new(Receiver::new())));
        info!("Joined {}", connect_to.mention());

        match ffmpeg(ding.file()) {
            Ok(stream) => handler.play(stream),
            Err(err) => error!("Error opening ffmpeg source '{}': {}", ding.file(), err),
        }
    } else {
        info!("Error joining the channel");
    }
}
