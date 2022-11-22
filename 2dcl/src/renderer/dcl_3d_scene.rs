use serde::{Deserialize, Serialize};
use dcl_common::{Parcel};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DCL3dScene {

   pub display: Display,
   pub contact: Option<Contact>,
   pub owner: String,
   pub scene: Scene,
   pub communications: Option<Communications>,
   pub policy: Option<Policy>,
   pub required_permissions: Option<Vec<String>>,
   pub main: String,
   pub tags: Option<Vec<String>>,
//   pub spawnPoints: Option<Vec<SpawnPoints>>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Display {
   pub title: String,
   pub description: Option<String>,
   pub navmap_thumbnail: Option<String>,
   pub favicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Contact {
   pub name: Option<String>,
   pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Scene {
   pub parcels: Vec<Parcel>,
   pub base: Parcel
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Communications {
   //pub _type: String,
   pub signalling: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Policy {
   pub content_rating: String,
   pub fly: bool,
   pub voice_enabled: bool,
   pub blacklist: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpawnPoints {
   pub name: String,
   pub default: bool,
   pub position: Position,
   pub camera_target: CameraTarget,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
   pub x: [f32;2],
   pub y: [f32;2],
   pub z: [f32;2],
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CameraTarget {
   pub x: i16,
   pub y: i16,
   pub z: i16,
}

