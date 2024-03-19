use poise::command;
use crate::{Context, Error};
use crate::utils::{get_channel_id, try_set_mutex};

/// Update the channel where moderation logs are sent
#[command(
prefix_command,
track_edits,
slash_command,
rename="log-channel",
required_permissions = "ADMINISTRATOR"
)]
pub async fn set_log_channel(
    ctx: Context<'_>,
    channel_name: String
) -> Result<(), Error> {
    println!("Received command to change logging channel to {channel_name}");

    let response = update_channel(&ctx, channel_name)?;
    ctx.say(response).await?;
    Ok(())
}

fn update_channel(ctx: &Context, channel_name: String) -> Result<String, Error> {
    let guild = match ctx.guild() {
        Some(guild_ref) => guild_ref,
        None => return Ok("Unable to find server ID, channel not updated".into())
    };
    match get_channel_id(&guild.channels, &channel_name) {
        Some(_) => {
            try_set_mutex(&ctx.data().log_channel, Some(channel_name.clone()))?;
            Ok(format!("Updated logging channel to #{channel_name}"))
        },
        None => Ok("Unable to update the logging channel - entered channel may not exist".into())
    }
}