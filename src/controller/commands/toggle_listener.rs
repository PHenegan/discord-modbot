use poise::command;
use crate::{Context, Error};
use crate::utils::try_set_mutex;

/// Enable listener to check messages for discord links
#[command(prefix_command,
track_edits,
slash_command,
rename="enable-checks",
required_permissions = "ADMINISTRATOR"
)]
pub async fn enable_listener(ctx: Context<'_>) -> Result<(), Error> {
    println!("Received 'enable-checks' command");
    try_set_mutex(&ctx.data().enable_listener, true)?;
    ctx.say("Enabled banning on discord link detection").await?;
    Ok(())
}

/// Disable listener to check messages for discord links
#[command(prefix_command,
track_edits,
slash_command,
rename="disable-checks",
required_permissions = "ADMINISTRATOR"
)]
pub async fn disable_listener(ctx: Context<'_>) -> Result<(), Error> {
    println!("Received 'disable-checks' command");
    try_set_mutex(&ctx.data().enable_listener, false)?;
    ctx.say("Disabled banning on discord link detection").await?;
    Ok(())
}