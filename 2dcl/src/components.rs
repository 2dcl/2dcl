use bevy::prelude::*;
use bevy::tasks::Task;
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::renderer::animations::Animation;
use crate::renderer::player::LevelChangeStackData;

#[derive(Component)]
pub struct DownloadingScene {
    pub task: Task<Option<Vec<PathBuf>>>,
    pub parcels: Vec<Parcel>,
}

#[derive(Component)]
pub struct Loading {
    pub animation_alpha: f32,
    pub animation_forward: bool,
    pub parcels: Vec<Parcel>,
}

#[derive(Component)]
pub struct LoadingSpriteRenderer {
    pub task: Task<(Handle<Image>, Vec2)>,
    pub sprite_renderer_component: dcl2d_ecs_v1::components::SpriteRenderer,
    pub transform: Transform,
    pub parcels: Vec<Parcel>,
    pub level_id: usize,
}

#[derive(Component)]
pub struct GettingNewestScenes {
    pub task: Task<Option<(Vec<catalyst::entity_files::SceneFile>, Vec<Parcel>)>>,
}

#[derive(Debug, Component, Clone)]
pub struct BoxCollider {
    pub center: Vec2,
    pub size: Vec2,
    pub collision_type: CollisionType,
    pub parcels: Vec<Parcel>,
}

#[derive(Debug, Component, Clone, PartialEq)]
pub struct LevelChange {
    pub spawn_point: Vec2,
    pub level: usize,
    pub parcels: Vec<Parcel>,
}

#[derive(Debug, Component, Reflect)]
pub struct Scene {
    pub name: String,
    #[reflect(ignore)]
    pub parcels: Vec<Parcel>,
    #[reflect(ignore)]
    pub timestamp: SystemTime,
    #[reflect(ignore)]
    pub serialized_data: Vec<u8>,
    pub path: PathBuf,
    pub is_default: bool,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            name: String::default(),
            parcels: Vec::default(),
            timestamp: SystemTime::now(),
            serialized_data: Vec::default(),
            path: PathBuf::default(),
            is_default: false,
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
    pub input_state: PlayerInputState,
    pub current_level: usize,
    pub current_parcel: Parcel,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerInputState {
    Normal,
    LoadingLevel(LevelChange),
    ExitingLevel,
}

#[derive(Component, Default)]
pub struct InteractIcon;

#[derive(Component, Default)]
pub struct SpriteRenderer {
    pub default_color: Color,
    pub parcels_overlapping: Vec<Parcel>,
    pub parent_parcels: Vec<Parcel>,
    pub is_on_top_of_player: bool,
    pub is_on_top_of_player_parcel: bool,
    pub transparency_alpha: f32,
}
