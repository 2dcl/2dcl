use std::path::Path;

use bevy::prelude::*;

mod scene_hot_reload;
use scene_hot_reload::{SceneAsset, SceneAssetLoader, SceneHotReloadPlugin};

mod level_switch;
use level_switch::level_switch;

mod collider_debugger;
use collider_debugger::collider_debugger;

mod manual_refresh;
use manual_refresh::manual_refresh;

mod ui;

use crate::{bundles::loading_animation, renderer::scene_loader::loading_sprites_task_handler};

use self::manual_refresh::RefreshData;

pub fn preview<T, U>(source_path: T, destination_path: U)
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    let source_path = source_path.as_ref();
    let destination_path = destination_path.as_ref();

    // compile
    scene_compiler::compile(source_path, destination_path).unwrap();

    let mut src_abs_path = std::fs::canonicalize(".").unwrap();
    src_abs_path.push(source_path);

    let mut dst_abs_path = std::fs::canonicalize(".").unwrap();
    dst_abs_path.push(destination_path);

    // run preview
    preview_scene(src_abs_path.to_path_buf(), dst_abs_path.to_path_buf());
}

pub fn preview_scene(source_path: std::path::PathBuf, destination_path: std::path::PathBuf) {
    let mut app = App::new();
    crate::renderer::setup(
        &mut app,
        "2dcl - Scene Preview".to_string(),
        &destination_path,
    );

    app.add_plugins(SceneHotReloadPlugin)
        .insert_resource(RefreshData {
            source_path,
            destination_path,
        })
        .add_systems(
            Update,
            (
                level_switch,
                collider_debugger,
                manual_refresh,
                ui::toggle_ui,
                loading_animation,
                loading_sprites_task_handler,
            ),
        )
        .add_asset::<SceneAsset>()
        .init_asset_loader::<SceneAssetLoader>()
        //.init_resource::<bevy_console::ConsoleOpen>()
        .run();
}
