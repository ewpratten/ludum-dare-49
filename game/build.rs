use anyhow::Result;
use vergen::{Config, ShaKind, vergen};

fn main() -> Result<()> {

    // Reads local Git information to inject into the executable at build time
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config)
}
