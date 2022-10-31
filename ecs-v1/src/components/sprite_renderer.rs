use serde::{Serialize, Deserialize};
use crate::Anchor;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpriteRenderer {
    pub sprite: String,
    pub color: [f32; 4],
    pub layer: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: Anchor
}
