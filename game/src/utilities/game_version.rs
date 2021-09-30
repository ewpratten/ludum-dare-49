use cfg_if::cfg_if;
use pkg_version::*;

const GAME_VERSION_MAJOR: u32 = pkg_version_major!();
const GAME_VERSION_MINOR: u32 = pkg_version_minor!();
const GAME_VERSION_PATCH: u32 = pkg_version_patch!();

/// Get the game version as a string
#[inline]
pub fn get_version_string() -> String {
    format!(
        "v{}.{}.{}{}",
        GAME_VERSION_MAJOR,
        GAME_VERSION_MINOR,
        GAME_VERSION_PATCH,
        {
            cfg_if! {
            if #[cfg(debug_assertions)] {
                "-debug"
            } else {
                ""
            }}
        }
    )
}
