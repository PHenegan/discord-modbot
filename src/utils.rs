use std::collections::HashMap;
use std::sync::Mutex;
use poise::serenity_prelude::{ChannelId, GuildChannel, Message};
use crate::controller::errors::AccessError;

pub fn has_discord_link(msg: &Message) -> bool {
    msg.embeds.iter()
        .filter_map(|embed| embed.url.clone())
        .any(|link| link.contains("discord.gg"))
}

pub fn get_channel_id( channels: &HashMap<ChannelId, GuildChannel>, channel_name: &String) -> Option<ChannelId> {
    channels.iter()
        .map(|(_id, channel)| channel)
        .find(|channel| channel.name == *channel_name)
        .map(|channel| channel.id)
}

pub fn try_set_mutex<T>(mutex: &Mutex<T>, value: T) -> Result<(), AccessError> {
    let mut lock = mutex.lock()
        .map_err(|_err| AccessError)?;
    *lock = value;
    Ok(())
}

pub fn try_get_mutex<T : Clone>(mutex: &Mutex<T>) -> Result<T, AccessError> {
    let lock = mutex.lock()
        .map_err(|_err| AccessError)?;
    let result = (*lock).clone();
    Ok(result)
}