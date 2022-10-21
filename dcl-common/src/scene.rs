use serde::{Deserialize, Serialize};
use super::Parcel;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Scene {
   pub name: String,
   pub entities: Vec<Entity>,
   pub parcels: Vec<Parcel>,
   pub path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub components: Vec<Component>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Component{
    Transform(Transform),
    SpriteRenderer(SpriteRenderer),
    CircleCollider(CircleCollider),
    BoxCollider(BoxCollider),
    AlphaCollider(AlphaCollider),
    AsepriteAnimation(AsepriteAnimation),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AsepriteAnimation {
    pub json_path: String,
    pub starting_state: String,
    pub color: [f32; 4],
    pub layer: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: Anchor
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transform {
    pub location: [f32; 2],
    pub rotation: [f32; 3],
    pub scale: [f32; 2],
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpriteRenderer {
    pub sprite: String,
    pub color: [f32; 4],
    pub layer: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: Anchor
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Anchor{
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    Custom([i16; 2]),
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CircleCollider {
    pub center: [i16; 2],
    pub raius: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxCollider {
    pub center: [i16; 2],
    pub size: [i16; 2],
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlphaCollider {
    pub sprite: String,
    pub channel: i32,
    pub anchor: Anchor
}