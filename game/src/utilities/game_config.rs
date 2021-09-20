//! Contains the general configuration data for the game

use rust_embed::EmbeddedFile;

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub github: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameConfig {
    pub name: String,
    pub authors: Vec<Author>,
}

impl GameConfig {
    /// Load from a file
    pub fn load(file: EmbeddedFile) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&file.data)
    }
}
