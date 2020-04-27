use log::{error, info};
use serenity::{
    client::{bridge::voice::ClientVoiceManager, Context},
    model::{
        guild::Guild,
        prelude::{ChannelId, ChannelType, GuildId},
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

macro_rules! try_ok {
    ($yeeter:expr) => {
        match $yeeter {
            Ok(good) => good,
            Err(_) => return None,
        }
    };
}

macro_rules! try_some {
    ($yeeter:expr) => {
        match $yeeter {
            Some(good) => good,
            None => return None,
        }
    };
}

fn find_dings(ctx: &Context, guild: Arc<RwLock<Guild>>) -> Option<(GuildId, ChannelId)> {
    let guild = guild.read();

    let chans = try_ok!(guild.channels(&ctx.http));

    let chan = try_some!(chans
        .into_iter()
        .filter(|(_, chan)| matches!(
            (chan.kind, chan.name.as_str()),
            (ChannelType::Voice, "Dings") | (ChannelType::Voice, "Wordwar")
        ))
        .next());

    Some((guild.id, chan.0))
}

pub(crate) fn ding(ding: Ding, ctx: &Context, guild: Arc<RwLock<Guild>>) {
    let (guild_id, connect_to) = match find_dings(ctx, guild) {
        Some(c) => c,
        None => {
            error!("No ding channel for guild");
            return;
        }
    };

    let manager_lock = ctx
        .data
        .read()
        .get::<Manager>()
        .cloned()
        .expect("Expected voice::Manager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.join(guild_id, connect_to) {
        info!("Joined dings channel {}/{}", guild_id, connect_to);

        let dingfile = ding.file();
        match ffmpeg(dingfile) {
            Ok(stream) => {
                info!("Playing {} for {:?}", dingfile, ding);
                handler.play_returning(stream).lock().play();
            }
            Err(err) => error!("Error opening ffmpeg source '{}': {}", dingfile, err),
        }
    } else {
        info!("Error joining the channel");
    }
}
