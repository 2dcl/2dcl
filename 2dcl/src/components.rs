use bevy::prelude::*;
use bevy::tasks::Task;
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::bundles::LoadingSpriteData;
use crate::renderer::animations::Animation;
use crate::renderer::player::LevelChangeStackData;

#[derive(Component)]
pub struct DownloadingScene {
    pub task: Task<Option<Vec<PathBuf>>>,
    pub parcels: Vec<Parcel>,
}

#[derive(Component)]
pub struct LoadingSprite {
    pub task: Task<LoadingSpriteData>,
    pub scene_entity: Entity,
}

#[derive(Debug, Component, Clone)]
pub struct CircleCollider {
    pub center: Vec2,
    pub radius: u32,
}

#[derive(Debug, Component, Clone)]
pub struct BoxCollider {
    pub center: Vec2,
    pub size: Vec2,
    pub collision_type: CollisionType,
}

#[derive(Debug, Component, Clone)]
pub struct LevelChange {
    pub spawn_point: Vec2,
    pub level: usize,
}

#[derive(Debug, Component)]
pub struct Scene {
    pub name: String,
    pub parcels: Vec<Parcel>,
    pub timestamp: SystemTime,
    pub serialized_data: Vec<u8>,
    pub path: PathBuf,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            name: String::default(),
            parcels: Vec::default(),
            timestamp: SystemTime::now(),
            serialized_data: Vec::default(),
            path: PathBuf::default(),
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Level {
    pub name: String,
    pub timestamp: SystemTime,
    pub id: usize,
    pub spawn_point: Vec2,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            name: String::default(),
            timestamp: SystemTime::now(),
            id: 0,
            spawn_point: Vec2::default(),
        }
    }
}

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
