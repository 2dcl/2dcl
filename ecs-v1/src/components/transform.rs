use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transform {
    pub location: [f32; 2],
    pub rotation: [f32; 3],
    pub scale: [f32; 2],
}
