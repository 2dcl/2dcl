use crate::entity::EntityType;
use crate::ContentId;
use crate::EntityId;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct SceneFile {
    pub id: Option<EntityId>,
    pub version: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub kind: EntityType,
    pub pointers: Vec<String>,
    pub timestamp: u128,
    pub content: Vec<ContentFile>,
    pub metadata: Option<DCL3dScene>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ContentFile {
    #[serde(rename(deserialize = "file", serialize = "file"))]
    pub filename: PathBuf,
    #[serde(rename(deserialize = "hash", serialize = "hash"))]
    pub cid: ContentId,
}

use dcl_common::Parcel;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DCL3dScene {
    pub display: Display,
    pub contact: Option<Contact>,
    pub owner: Option<String>,
    pub scene: Scene,
    pub communications: Option<Communications>,
    pub policy: Option<Policy>,
    pub required_permissions: Option<Vec<String>>,
    pub main: Option<String>,
    pub tags: Option<Vec<String>>,
    //   pub spawnPoints: Option<Vec<SpawnPoints>>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Display {
    pub title: String,
    pub description: Option<String>,
    pub navmap_thumbnail: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Scene {
    pub parcels: Vec<Parcel>,
    pub base: Parcel,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Communications {
    //pub _type: String,
    pub signalling: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Policy {
    pub content_rating: Option<String>,
    pub fly: Option<bool>,
    pub voice_enabled: Option<bool>,
    pub blacklist: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpawnPoints {
    pub name: Option<String>,
    pub default: Option<bool>,
    pub position: Option<Position>,
    pub camera_target: Option<CameraTarget>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub x: [f32; 2],
    pub y: [f32; 2],
    pub z: [f32; 2],
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CameraTarget {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scene_file_deserializes_from_json() {
        let json = include_str!("../fixtures/scene.json");
        let scene_file: SceneFile = serde_json::from_str(json).unwrap();
        assert_eq!(scene_file.kind, EntityType::Scene);
    }

    #[test]
    fn content_file_deserializes_from_json() {
        let json = "{\"file\":\"file.txt\",\"hash\":\"hash\"}";

        let content_file: ContentFile = serde_json::from_str(json).unwrap();

        let mut expected = PathBuf::new();
        expected.set_file_name("file.txt");
        assert_eq!(content_file.filename, expected);
        assert_eq!(content_file.cid, ContentId::new("hash"));
    }
}
