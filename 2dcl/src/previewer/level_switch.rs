use bevy::ecs::system::Query;
use bevy::input::Input;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::prelude::KeyCode;
use crate::previewer::{Previewer, PreviewerState};
use crate::renderer::PlayerComponent;

pub fn level_switch(
    keyboard: Res<Input<KeyCode>>,
    mut previewer: ResMut<Previewer>,
    mut player_query: Query<&mut PlayerComponent>
) {
  if previewer.state == PreviewerState::Idle && keyboard.just_pressed(KeyCode::L) {
    previewer.state = PreviewerState::LevelSwitch;
  }

  if previewer.state == PreviewerState::LevelSwitch {
    if let Ok(mut player) = player_query.get_single_mut() {
      for key_pressed in  keyboard.get_just_pressed() {
        match key_pressed {
          KeyCode::Key1 => { switch(0, &mut previewer, &mut player) }
          KeyCode::Key2 => { switch(1, &mut previewer, &mut player) }
          KeyCode::Key3 => { switch(2, &mut previewer, &mut player) }
          KeyCode::Key4 => { switch(3, &mut previewer, &mut player) }
          KeyCode::Key5 => { switch(4, &mut previewer, &mut player) }
          KeyCode::Key6 => { switch(5, &mut previewer, &mut player) }
          KeyCode::Key7 => { switch(6, &mut previewer, &mut player) }
          KeyCode::Key8 => { switch(7, &mut previewer, &mut player) }
          KeyCode::Key9 => { switch(8, &mut previewer, &mut player) }
          KeyCode::Key0 => { switch(9, &mut previewer, &mut player) }
          KeyCode::L => {}
          _ => { 
            println!("None.");
            previewer.state = PreviewerState::Idle; }
        }  
      }
    }
  }
}


fn switch(level: usize, previewer: &mut ResMut<Previewer>, player: &mut PlayerComponent) {
  player.current_level = level;
  previewer.state = PreviewerState::Idle;
}
