use serde::{Serialize, Deserialize};
use crate::Component;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entity {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    pub components: Vec<Component>,
}
