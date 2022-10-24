use serde::{Deserialize, Serialize};
use dcl_common::{Parcel};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DCLScene {

   pub display: Display,
   pub contact: Contact,
   pub owner: String,
   pub scene: Scene,
   pub communications: Option<Communications>,
   pub policy: Option<Policy>,
   pub requiredPermissions: Option<Vec<String>>,
   pub main: String,
   pub tags: Option<Vec<String>>,
//   pub spawnPoints: Option<Vec<SpawnPoints>>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Display {
   pub title: String,
   pub description: Option<String>,
   pub navmapThumbnail: Option<String>,
   pub favicon: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Contact {
   pub name: String,
   pub email: String,
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
   pub contentRating: String,
   pub fly: bool,
   pub voiceEnabled: bool,
   pub blacklist: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpawnPoints {
   pub name: String,
   pub default: bool,
   pub position: Position,
   pub cameraTarget: CameraTarget,
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

