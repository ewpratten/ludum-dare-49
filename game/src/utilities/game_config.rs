//! Contains the general configuration data for the game
//! This data is immutable, and should only be edited by hand

use raylib::color::Color;
use rust_embed::EmbeddedFile;

/// Defines one of the game's authors
#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTheme {
    pub red: Color,
    pub blue: Color,
    pub green: Color,
    pub yellow: Color,
    pub pink: Color,
    pub background: Color,
    pub white: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameConfig {
    pub name: String,
    pub base_window_size: (i32, i32),
    pub sentry_dsn: String,
    pub colors: ColorTheme,
    pub animation_fps: usize,

    #[serde(skip)]
    pub debug_view: bool
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
    pub bloom_samples: f32,
    pub bloom_quality: f32,
}

impl FinalShaderConfig {
    /// Load from a file
    pub fn load(file: EmbeddedFile) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&file.data)
    }
}
