use super::renderer::collision;
use bevy::prelude::*;

#[derive(Default, Clone, Resource)]
pub struct CollisionMap {
    pub tiles: Vec<collision::CollisionTile>,
    pub tile_size: f32,
}
