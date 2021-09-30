use std::io::Write;

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};
use tempfile::{tempfile, NamedTempFile};

/// Contains all game assets.
///
/// This uses macro magic to automatically embed the contents of `game/assets/` into the executable
/// file so we only have to distribute a single file, instead of a game and its assets separately
#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
pub struct StaticGameData;

#[derive(Debug, Error)]
pub enum ResourceLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not load embedded asset: {0}")]
    AssetNotFound(String),
    #[error("Generic error: {0}")]
    Generic(String),
}

pub fn load_texture_from_internal_data(
    raylib_handle: &RaylibHandle,
    thread: &RaylibThread,
    path: &str,
) -> Result<Texture2D, ResourceLoadError> {
    // Create a temp file path to work with
    let tmp_path = NamedTempFile::new()?.into_temp_path();

    // Unpack the raw image data to a real file on the local filesystem so raylib will read it correctly
    std::fs::write(
        tmp_path,
        &StaticGameData::get(path)
            .ok_or(ResourceLoadError::AssetNotFound(path.to_string()))?
            .data,
    )?;

    // Call through via FFI to re-load the file
    let texture = raylib_handle.load_texture(thread, tmp_path.to_str().unwrap()).map_err(|e| ResourceLoadError::Generic(e));

    // Close the file
    tmp_path.close()?;

    Ok(texture)
}
