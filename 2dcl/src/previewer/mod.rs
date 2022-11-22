use std::str::FromStr;
use std::path::PathBuf;

use notify::EventKind::Access;
use notify::event::AccessKind::Close;

use std::path::Path;
use notify::{Watcher, Event, RecursiveMode};
use scene_compiler;

use bevy::prelude::*;

mod scene_hot_reload;
use scene_hot_reload::{SceneAsset, SceneAssetLoader, SceneHotReloadPlugin };

pub fn preview<T,U>(source_path: T, destination_path: U) 
where
  T: AsRef<Path>,
  U: AsRef<Path>
{
  let source_path = source_path.as_ref();
  let destination_path = destination_path.as_ref();

  let mut src_abs_path =  std::fs::canonicalize(".").unwrap();
  src_abs_path.push(source_path);
  let mut dst_abs_path= std::fs::canonicalize(".").unwrap();
  dst_abs_path.push(destination_path);
  
  let mut watcher = notify::recommended_watcher(move|res| {
    match res {
      Ok(event) => {
        let Event { kind, paths, attrs: _ } = event;

        if let Access(Close(_)) = kind {
          for path in paths {
            if !path.starts_with(&dst_abs_path) && (
                path.ends_with("scene.json") || path.extension().unwrap().to_string_lossy() == "png"
              ) {
              println!("Reloading {}...", src_abs_path.display());

              match scene_compiler::compile(&src_abs_path, &dst_abs_path) {
                Err(error) => println!("Error compiling: {}", error),
                _ => {}
              }
            }
          }          
        }

      },
      Err(e) => println!("watch error: {:?}", e),
    }
  }).unwrap();

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(source_path, RecursiveMode::Recursive).unwrap();

  // compile
  scene_compiler::compile(&source_path, &destination_path).unwrap();

  // run preview
  preview_scene(destination_path.to_path_buf());
}

pub fn preview_scene(base_dir: std::path::PathBuf)
{
   
    std::env::set_current_dir(&base_dir).unwrap();
    let absolute_base_dir = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
    std::env::set_var("CARGO_MANIFEST_DIR", absolute_base_dir);

    let mut app = App::new();
    crate::renderer::setup(&mut app);

    app
        .add_plugin(SceneHotReloadPlugin)
        .add_asset::<SceneAsset>()
        .init_asset_loader::<SceneAssetLoader>()
        .run();
}



