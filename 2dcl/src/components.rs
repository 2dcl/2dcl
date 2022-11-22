use std::path::PathBuf;
use std::time::SystemTime;
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;
use bevy::prelude::*;
use bevy::tasks::Task;

#[derive(Component)]
pub struct DownloadingScene {
    pub task: Task<()>,
    pub parcels: Vec<Parcel>,
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
  pub level: usize,
  pub spawn_point: Vec2,
}

#[derive(Debug, Component)]
pub struct Scene
{
    pub name: String,
    pub parcels: Vec<Parcel>,
    pub timestamp: SystemTime,
    pub scene_data: Vec<u8>,
    pub path: PathBuf,
}

#[derive(Debug, Component, Clone)]
pub struct Level
{
    pub name: String,
    pub timestamp: SystemTime,
    pub id: usize,
}
