use poise::command;
use crate::{Context, Error};
use crate::controller::errors::LockError;
use crate::utils::get_channel_id;

#[command(prefix_command, track_edits, slash_command, rename="set-log-channel")]
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
            let mut current_channel = ctx.data().log_channel
                .lock()
                .map_err(|_err| LockError)?;
            *current_channel = Some(channel_name.clone());
            Ok(format!("Updated logging channel to #{channel_name}"))
        },
        None => Ok("Unable to update the logging channel - entered channel may not exist".into())
    }
}