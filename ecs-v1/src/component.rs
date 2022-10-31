use serde::{Serialize, Deserialize};
use crate::components::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Component{
    Transform(Transform),
    SpriteRenderer(SpriteRenderer),
    CircleCollider(CircleCollider),
    BoxCollider(BoxCollider),
    AlphaCollider(AlphaCollider),
}
