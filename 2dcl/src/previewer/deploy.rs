use bevy::prelude::*;

use super::manual_refresh::RefreshData;

pub fn deploy(keyboard: Res<Input<KeyCode>>, refresh_data: Res<RefreshData>) {
    if keyboard.just_pressed(KeyCode::F4) {
        crate::deploy::deploy(&refresh_data.destination_path).unwrap();
    }
}
