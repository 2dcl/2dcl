use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CircleCollider {
    pub center: [i16; 2],
    pub radius: i32,
}
