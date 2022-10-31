use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxCollider {
    pub center: [i16; 2],
    pub size: [i16; 2],
}
