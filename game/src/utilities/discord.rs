//! Discord Rich Presence utilities

use discord_sdk::ds::{Discord, User, Wheel};

pub struct DiscordRpcClient {
    pub discord: Discord,
    pub user: User,
    pub wheel: Wheel,
}