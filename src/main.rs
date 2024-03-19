mod controller;
mod utils;

use std::sync::Mutex;
use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::CustomError;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use crate::controller::commands::{disable_bans, disable_listener, enable_bans, enable_listener, set_log_channel};
use crate::controller::event_handler::event_handler;

struct Data {
    log_channel: Mutex<Option<String>>,
    enable_bans: Mutex<bool>,
    enable_listener: Mutex<bool>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let token = secret_store.get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' not found")?;
    let intents = GatewayIntents::non_privileged()
        .union(GatewayIntents::MESSAGE_CONTENT);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                set_log_channel(),
                enable_bans(),
                disable_bans(),
                enable_listener(),
                disable_listener()
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    log_channel: Mutex::new(Some("".into())),
                    enable_listener: Mutex::new(false),
                    enable_bans: Mutex::new(false)
                })
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(CustomError::new)?;
    Ok(client.into())
}