use bevy::prelude::*;

use std::path::PathBuf;


pub struct RefreshData
{
 pub source_path: PathBuf,
 pub destination_path: PathBuf,
}

pub fn manual_refresh(
  keyboard: Res<Input<KeyCode>>,
  refresh_data: Res<RefreshData>,

) {
    if keyboard.just_pressed(KeyCode::R) {
      scene_compiler::compile(&refresh_data.source_path, &refresh_data.destination_path).unwrap();
    }
}