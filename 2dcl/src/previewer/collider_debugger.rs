use bevy::ecs::system::Res;
use bevy::prelude::KeyCode;
use bevy::input::Input;

pub fn collider_debugger(
  keyboard: Res<Input<KeyCode>>
) {
  if keyboard.just_pressed(KeyCode::C) {
    println!("Draw Debug");
    // Draw Debug
  }

  if keyboard.just_released(KeyCode::C) {
    // Stop debug
    println!("Stop Debug");
  }

}
