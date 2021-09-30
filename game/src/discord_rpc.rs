use std::time::Duration;

use discord_sdk::activity::ActivityBuilder;
use tracing::{error, log::info};

use crate::utilities::discord::{rpc::DiscordError, DiscordConfig, DiscordRpcClient};

/// How long to wait before we give up on connecting to Discord.
const DISCORD_CONNECT_TIMEOUT_SECONDS: u64 = 5;

/// Try to connect to a local discord client for RPC, or return an error.
pub async fn try_connect_to_local_discord(
    config: &DiscordConfig,
) -> Result<DiscordRpcClient, DiscordError> {
    info!("Trying to locate and connect to a local Discord process for RPC. Will wait up to {} seconds before timing out", DISCORD_CONNECT_TIMEOUT_SECONDS);

    // Connect while wrapped in a tokio timeout
    let rpc_client = tokio::time::timeout(
        Duration::from_secs(DISCORD_CONNECT_TIMEOUT_SECONDS),
        DiscordRpcClient::new(config.app_id, discord_sdk::Subscriptions::ACTIVITY),
    )
    .await??;

    info!("Successfully connected to Discord");
    Ok(rpc_client)
}

/// If the discord client object exists, set rich presence, otherwise, do nothing.
pub async fn maybe_set_discord_presence(
    client: &Option<DiscordRpcClient>,
    activity: ActivityBuilder,
) -> Result<(), DiscordError> {
    if let Some(rpc) = client {
        rpc.set_rich_presence(activity).await?;
    }
    Ok(())
}
