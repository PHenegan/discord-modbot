use crate::{Data, Error};

use poise::serenity_prelude::{ Context, FullEvent };
use serenity::all::{ChannelId, GuildId, Message, User};
use crate::controller::errors::LockError;
use crate::utils::{get_channel_id, has_discord_link};

pub struct EventHandler<'a> {
    pub(crate) ctx: &'a Context,
    data: &'a Data
}

impl EventHandler<'_> {
    pub async fn handle_event(&self, event: &FullEvent) -> Result<(), Error> {
        match event {
            FullEvent::Message { new_message } => self.ban_discord_links(new_message).await?,
            _ => {}
        }
        Ok(())
    }

    async fn ban_discord_links(&self, message: &Message) -> Result<(), Error> {
        println!("Checking message '{:?}' for discord links", message);
        // ensures the bot doesn't crash if the guild ID does not exist
        let guild_id = match message.guild_id {
            Some(id) => id,
            None => return Ok(())
        };

        // return without doing anything if the message is fine
        if !has_discord_link(message) { //|| self.is_allowed_invites(&guild_id, &message.author).await {
            return Ok(());
        }

        // self.ctx.http.ban_user(
        //     guild_id,
        //     message.author.id,
        //     1,
        //     Some("Discord links are not allowed due to scam attacks"),
        // ).await?;
        println!("Found a discord link");

        let all_channels = guild_id.channels(&self.ctx.http).await?;

        let channel: Option<ChannelId> = {
            let channel_name = self.data.log_channel
                .lock()
                .map_err(|_err| LockError)?;
            channel_name.as_ref().map(|name| get_channel_id(&all_channels, &name)).flatten()
        };

        match channel {
            Some(channel) => {
                channel.say(
                    &self.ctx.http,
                    format!("Banned user '{}' for posting a Discord link", message.author.tag())
                ).await?;
            },
            None => {}
        };

        Ok(())
    }

    /// Allow discord link invites if the poster is an administrator
    async fn is_allowed_invites(&self, guild_id: &GuildId, user: &User) -> bool {
        let member_res = self.ctx.http.get_member(*guild_id, user.id).await;

        let member = match member_res {
            Ok(result) => result,
            Err(_) => return false
        };

        match member.permissions {
            Some(permissions) => permissions.administrator(),
            None => false
        }
    }
}
pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let handler: EventHandler = EventHandler { ctx, data };
    handler.handle_event(event).await
}