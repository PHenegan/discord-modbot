use std::collections::HashMap;
use serenity::all::{ChannelId, GuildChannel, Message};

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