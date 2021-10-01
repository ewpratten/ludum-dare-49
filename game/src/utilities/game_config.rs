//! Contains the general configuration data for the game
//! This data is immutable, and should only be edited by hand

use rust_embed::EmbeddedFile;

/// Defines one of the game's authors
#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameConfig {
    pub name: String,
    // pub authors: Vec<Author>,
    pub base_window_size: (i32, i32),
    pub sentry_dsn: String,
}

impl GameConfig {
    /// Load from a file
    pub fn load(file: EmbeddedFile) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&file.data)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinalShaderConfig {
    pub pixel_scale: f32,
    pub warp_factor: f32,
    pub scanline_darkness: f32,
}

impl FinalShaderConfig {
    /// Load from a file
    pub fn load(file: EmbeddedFile) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&file.data)
    }
}
