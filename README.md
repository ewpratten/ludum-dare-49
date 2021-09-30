# ludum-dare-49
[![Build](https://github.com/Ewpratten/ludum-dare-49/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/ludum-dare-49/actions/workflows/build.yml)
[![Clippy check](https://github.com/Ewpratten/ludum-dare-49/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/ludum-dare-49/actions/workflows/clippy.yml)
[![Ludum Dare 49](https://img.shields.io/badge/Ludum%20Dare-49-orange)](https://ldjam.com/events/ludum-dare/49/$261521)
[![Rust 1.57 nightly](https://img.shields.io/badge/Rust-1.57%20nightly-orange)](https://www.rust-lang.org/)
[![Made with Raylib](https://img.shields.io/badge/Made%20With-raylib-blue)](https://www.raylib.com/)

## The Team

This game is developed by a team of 4 students from *Sheridan College* and *Trent University*.

- [**Evan Pratten**](https://github.com/ewpratten)
  - Team lead
  - Software developer
  - Other LD games: [*Micromanaged Mike*](https://ldjam.com/events/ludum-dare/46/micromanaged-mike), [*Deep Breath*](https://github.com/ewpratten/ludum-dare-48)
- [**Carter Tomlenovich**](https://github.com/hyperliskdev)
  - Software developer
- [**Marcelo**](https://github.com/SNOWZ7Z)
  - Software developer
- [**Luna**](https://github.com/LuS404)
  - Software developer

## Directory Structure

- `game`: Contains the game code and assets
  - `src`: Rust code
    - [`lib.rs`](game/src/lib.rs): The game's main file
  - `assets`: Any files to be embedded directly into the final game executable (managed by [`game::utilities::datastore::StaticGameData`](game/src/utilities/datastore.rs) using the [`rust-embed`](https://github.com/pyros2097/rust-embed) library)
  - `Cargo.toml`: The game's dependencies
- `wrapper`: This is just a small hack to improve the compile times of the game. Don't mess with anything in here

## Building for release

These steps should only be followed by whoever is building the final game executables for release. This is *not needed* for development.

Firstly, ensure the docker images are built:

```sh
docker build -t ldjam_49_x86_64_unknown_linux_gnu_build_env -f ./bundle/docker/x86_64-unknown-linux-gnu.dockerfile .
```

Then, build in release mode for targeted platforms:

```sh
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
```

The resulting binaries will be in the `target` directory. Make sure to rename the executables before release.
