use bevy::prelude::*;
use dcl_common::Parcel;

use super::{animations::Animation, player::LevelChangeStackData};

#[derive(Component, Debug)]
pub struct Animator {
  pub current_animation: Animation,
  pub animations: Vec<Animation>,
  pub atlas: Handle<TextureAtlas>,
  pub scale: f32,
  pub frame_durations: Vec<f32>,
  pub timer: Timer,
  pub animation_queue: Vec<Animation>,
}


#[derive(Component, Debug)]
pub struct Player {
  pub speed: f32,
  pub collider_size: Vec2,
  pub level_change_stack: Vec<LevelChangeStackData>,
  pub current_level: usize,
  pub current_parcel: Parcel,
}

#[derive(Component, Default)]
pub struct InteractIcon;