use crate::entity::EntityType;
use crate::scene::Scene;
use crate::ContentId;
use crate::EntityId;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SceneFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EntityId>,
    pub version: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub kind: EntityType,
    pub pointers: Vec<String>,
    pub timestamp: u128,
    pub content: Vec<ContentFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Scene>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EntityFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EntityId>,
    pub version: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub kind: EntityType,
    pub pointers: Vec<String>,
    pub timestamp: u128,
    pub content: Vec<ContentFile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContentFile {
    #[serde(rename(deserialize = "file", serialize = "file"))]
    pub filename: PathBuf,
    #[serde(rename(deserialize = "hash", serialize = "hash"))]
    pub cid: ContentId,
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
