use bevy::prelude::*;
use bevy::tasks::Task;
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;
use std::path::PathBuf;
use std::time::SystemTime;

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

pub struct LoadingSpriteData {
    pub sprite: Sprite,
    pub image: Handle<Image>,
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

#[derive(Debug, Component, Clone)]
pub struct Level {
    pub name: String,
    pub timestamp: SystemTime,
    pub id: usize,
    pub spawn_point: Vec2,
}
