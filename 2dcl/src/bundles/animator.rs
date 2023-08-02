use std::{collections::HashMap, time::Duration};

use crate::{
    components,
    renderer::animation::{Animation, AnimationState},
};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Bundle)]
pub struct Animator {
    pub sprite_sheet: SpriteSheetBundle,
    pub animator: components::Animator,
}

impl Animator {
    pub fn from_json(
        json_str: &str,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Result<Animator, serde_json::Error> {
        let json = serde_json::from_str::<AnimatorJson>(json_str)?;
        let mut spritesheet_path = std::env::current_exe().unwrap_or_default();
        spritesheet_path.pop();
        spritesheet_path.push("assets");
        spritesheet_path.push(json.spritesheet);
        let texture_handle = asset_server.load(spritesheet_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(json.tile_size_x, json.tile_size_y),
            json.columns,
            json.rows,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let duration = match json.animations.get(&json.default_state) {
            Some(animation) => Duration::from_secs_f32(1. / animation.frame_rate),
            None => Duration::default(),
        };

        Ok(Animator {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            },
            animator: components::Animator {
                current_state: json.default_state,
                state_queue: Vec::default(),
                animations: json.animations,
                timer: Timer::new(duration, TimerMode::Repeating),
            },
        })
    }
}

#[derive(Deserialize)]
struct AnimatorJson {
    spritesheet: String,
    tile_size_x: f32,
    tile_size_y: f32,
    columns: usize,
    rows: usize,
    animations: HashMap<AnimationState, Animation>,
    default_state: AnimationState,
}
