//! Discord Rich Presence utilities

use discord_sdk::{
    activity::{Activity, ActivityBuilder},
    user::User,
    wheel::Wheel,
    Discord, DiscordApp, Subscriptions,
};
use tokio::time::error::Elapsed;
use tracing::info;

#[derive(Debug, Error)]
pub enum DiscordError {
    #[error(transparent)]
    Sdk(#[from] discord_sdk::Error),
    #[error(transparent)]
    AwaitConnection(#[from] tokio::sync::watch::error::RecvError),
    #[error("Could not connect")]
    Connection,
    #[error(transparent)]
    ConnectionTimeout(#[from] Elapsed),
}

/// The client wrapper for Discord RPC
pub struct DiscordRpcClient {
    pub discord: Discord,
    pub user: User,
    pub wheel: Wheel,
}

impl DiscordRpcClient {
    /// Creates a new `DiscordRpcClient`
    pub async fn new(app_id: i64, subscriptions: Subscriptions) -> Result<Self, DiscordError> {
        // Create a new wheel
        let (wheel, handler) = Wheel::new(Box::new(|err| {
            tracing::error!(error = ?err, "encountered an error");
        }));
        let mut user = wheel.user();

        // Create the client
        let discord = Discord::new(
            DiscordApp::PlainId(app_id),
            subscriptions,
            Box::new(handler),
        )?;

        // Wait for the discord handshake
        info!("Waiting for Discord client handshake");
        user.0.changed().await?;
        info!("Discord handshake success");

        // Fetch the final user object
        let user = match &*user.0.borrow() {
            discord_sdk::wheel::UserState::Connected(u) => Ok(u.clone()),
            discord_sdk::wheel::UserState::Disconnected(_) => Err(DiscordError::Connection),
        }?;

        Ok(Self {
            discord,
            user,
            wheel,
        })
    }

    /// Clears the user rich presence
    #[allow(dead_code)]
    pub async fn clear_rich_presence(&self) -> Result<Option<Activity>, discord_sdk::Error> {
        puffin::profile_function!();
        self.discord
            .update_activity(ActivityBuilder::default())
            .await
    }

    /// Sets the user rich presence
    pub async fn set_rich_presence(
        &self,
        activity: ActivityBuilder,
    ) -> Result<Option<Activity>, discord_sdk::Error> {
        puffin::profile_function!();
        self.discord.update_activity(activity).await
    }
}
