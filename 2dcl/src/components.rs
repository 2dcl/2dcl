use crate::renderer::animation::{Animation, AnimationState};
use crate::renderer::player::LevelChangeStackData;
use bevy::prelude::*;
use bevy::tasks::Task;
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

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

#[derive(Debug)]
pub struct TimeStamp(pub SystemTime);

impl TimeStamp {
    fn now() -> Self {
        Self(SystemTime::now())
    }
}

impl Default for TimeStamp {
    fn default() -> Self {
        Self(SystemTime::now())
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Scene {
    pub name: String,
    #[reflect(ignore)]
    pub parcels: Vec<Parcel>,
    #[reflect(ignore)]
    pub timestamp: TimeStamp,
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
            timestamp: TimeStamp::now(),
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
pub struct Player {
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

#[derive(Component)]
pub struct Animator {
    pub current_state: AnimationState,
    pub state_queue: Vec<AnimationState>,
    pub animations: HashMap<AnimationState, Animation>,
    pub timer: Timer,
}

impl Animator {
    pub fn get_animation(&self, state: &AnimationState) -> Option<&Animation> {
        self.animations.get(state)
    }
    pub fn get_current_animation(&self) -> Option<&Animation> {
        self.get_animation(&self.current_state)
    }
    pub fn update_state(&mut self, new_state: AnimationState) {
        if new_state == self.current_state {
            return;
        }
        self.current_state = new_state;
        if let Some(current_animation) = self.get_current_animation() {
            self.timer
                .set_duration(Duration::from_secs_f32(1. / current_animation.frame_rate));
        }
        self.timer.set_elapsed(self.timer.duration());
    }
}
