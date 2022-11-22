use bevy::ecs::system::Query;
use bevy::input::Input;
use bevy::prelude::KeyCode;
use bevy::ecs::system::Res;
use crate::renderer::PlayerComponent;

pub fn level_switch(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut PlayerComponent>
) {
  if let Ok(mut player) = player_query.get_single_mut() {
    for key_pressed in  keyboard.get_just_pressed() {
      match key_pressed {
        KeyCode::Key1 => { switch(0, &mut player) }
        KeyCode::Key2 => { switch(1, &mut player) }
        KeyCode::Key3 => { switch(2, &mut player) }
        KeyCode::Key4 => { switch(3, &mut player) }
        KeyCode::Key5 => { switch(4, &mut player) }
        KeyCode::Key6 => { switch(5, &mut player) }
        KeyCode::Key7 => { switch(6, &mut player) }
        KeyCode::Key8 => { switch(7, &mut player) }
        KeyCode::Key9 => { switch(8, &mut player) }
        KeyCode::Key0 => { switch(9, &mut player) }
        _ => {}
      }
    }
  }
}

fn switch(level: usize, player: &mut PlayerComponent) {
  player.current_level = level;
}
