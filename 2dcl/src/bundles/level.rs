use crate::components;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Level {
    pub name: Name,
    pub visibility: VisibilityBundle,
    pub transform: TransformBundle,
    pub level: components::Level,
}
