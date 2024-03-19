use poise::command;
use crate::{Context, Error};
use crate::utils::try_set_mutex;

/// Enable banning when bannable messages are found
#[command(prefix_command,
track_edits,
slash_command,
rename="enable-bans",
required_permissions = "ADMINISTRATOR"
)]
pub async fn enable_bans(ctx: Context<'_>) -> Result<(), Error> {
    println!("Received 'enable-bans' command");
    try_set_mutex(&ctx.data().enable_bans, true)?;
    ctx.say("Enabled banning on discord link detection").await?;
    Ok(())
}

/// Disable banning when bannable messages are found
#[command(
prefix_command,
track_edits,
slash_command,
rename="disable-bans",
required_permissions = "ADMINISTRATOR"
)]
pub async fn disable_bans(ctx: Context<'_>) -> Result<(), Error> {
    println!("Received 'disable-bans' command");
    try_set_mutex(&ctx.data().enable_bans, false)?;
    ctx.say("Disabled banning on discord link detection").await?;
    Ok(())
}