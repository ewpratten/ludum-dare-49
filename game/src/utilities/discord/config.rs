use rust_embed::EmbeddedFile;

#[derive(Debug, Clone, Deserialize)]
pub struct DiscordConfig {
    pub app_id: i64,
}

impl DiscordConfig {
    /// Load from a file
    pub fn load(file: EmbeddedFile) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&file.data)
    }
}
