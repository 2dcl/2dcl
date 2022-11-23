use bevy::prelude::Vec2;

pub const MIN_RENDERING_DISTANCE_IN_PARCELS: i16 = 3;
pub const MAX_RENDERING_DISTANCE_IN_PARCELS: i16 = 6;
pub const PARCEL_SIZE_X: f32 = 500.0;
pub const PARCEL_SIZE_Y: f32 = 500.0;
pub const PLAYER_SCALE: f32 = 1.0;
pub const PLAYER_SPEED: f32 = 400.0;
pub const PLAYER_COLLIDER: Vec2 = Vec2::new(18.0,20.0);
pub const CAMERA_SCALE: f32 = 1.0;
pub const ITERACT_ICON_HEIGHT: f32 = 100.0;