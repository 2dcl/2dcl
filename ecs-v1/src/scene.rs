use serde::{Serialize, Deserialize};
use dcl_common::Parcel;
use crate::Entity;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Scene {
   #[serde(skip)]
   pub id: usize,
   pub name: String,
   pub entities: Vec<Entity>,
   pub parcels: Vec<Parcel>,
}
