
use notify::EventKind::Access;
use notify::event::AccessKind::Close;

use std::path::Path;
use notify::{Watcher, Event, RecursiveMode};
use crate::renderer;
use scene_compiler;

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
            if path.ends_with("scene.json") || path.extension().unwrap().to_string_lossy() == "png"  {
              scene_compiler::compile(&src_abs_path, &dst_abs_path).unwrap();
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
  renderer::preview_scene(destination_path.to_path_buf());
}
