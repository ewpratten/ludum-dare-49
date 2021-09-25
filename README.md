# ludum-dare-49

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
