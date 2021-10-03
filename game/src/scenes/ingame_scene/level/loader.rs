use raylib::{RaylibHandle, RaylibThread};

use crate::{
    utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        world_paint_texture::WorldPaintTexture,
    },
    StaticGameData,
};

use super::Level;

pub fn load_all_levels(
    raylib_handle: &mut RaylibHandle,
    thread: &RaylibThread,
) -> Result<Vec<Level>, ResourceLoadError> {
    // Get a listing of all levels we have
    let level_names: Vec<String> = serde_json::from_str(
        &String::from_utf8(
            StaticGameData::get("levels/levels.json")
                .expect("Could not load levels.json")
                .data
                .into(),
        )
        .unwrap(),
    )?;

    // Build a level list
    let mut levels = Vec::new();

    for level_name in &level_names {
        let zones = serde_json::from_str(
            &String::from_utf8(
                StaticGameData::get(&format!("levels/{}/zones.json", level_name))
                    .unwrap()
                    .data
                    .into(),
            )
            .unwrap(),
        )?;
        levels.push({
            let mut l = Level {
                name: level_name.to_string(),
                background_tex: WorldPaintTexture::new(load_texture_from_internal_data(
                    raylib_handle,
                    thread,
                    &format!("levels/{}/background.png", level_name),
                )?),
                platform_tex: load_texture_from_internal_data(
                    raylib_handle,
                    thread,
                    &format!("levels/{}/platforms.png", level_name),
                )?,
                appearing_platform_tex: load_texture_from_internal_data(
                    raylib_handle,
                    thread,
                    &format!("levels/{}/appearing_platforms.png", level_name),
                )?,
                disappearing_platform_tex: load_texture_from_internal_data(
                    raylib_handle,
                    thread,
                    &format!("levels/{}/disappearing_platforms.png", level_name),
                )?,
                colliders: serde_json::from_str(
                    &String::from_utf8(
                        StaticGameData::get(&format!("levels/{}/colliders.json", level_name))
                            .unwrap()
                            .data
                            .into(),
                    )
                    .unwrap(),
                )?,
                zones,
            };
            for a in &l.zones.appear {
                l.colliders.push(a.clone());
            }
            l
        });
    }
    Ok(levels)
}
