use std::path::PathBuf;
use std::str::FromStr;

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
    preview_scene(src_abs_path.to_path_buf(),dst_abs_path.to_path_buf());
}

pub fn preview_scene(source_path: std::path::PathBuf, destination_path: std::path::PathBuf) {

    std::env::set_current_dir(&destination_path).unwrap();
    let absolute_base_dir = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
    std::env::set_var("CARGO_MANIFEST_DIR", absolute_base_dir);

    let mut app = App::new();
    crate::renderer::setup(&mut app);

    app.add_plugin(SceneHotReloadPlugin)
        .insert_resource(RefreshData{ source_path, destination_path })
        .add_system(level_switch)
        .add_system(collider_debugger)
        .add_system(manual_refresh)
        .add_asset::<SceneAsset>()
        .init_asset_loader::<SceneAssetLoader>()
        .run();
}
