use dcl_common::Parcel;
use crate::EntityId;
use std::path::PathBuf;
use crate::ContentId;
use serde::Deserialize;
use crate::entity::EntityType;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct SceneFile {
    pub id: EntityId,
    pub version: String,
    #[serde(rename(deserialize = "type"))]
    pub kind: EntityType,
    pub pointers: Vec<Parcel>,
    pub timestamp: u64,
    pub content: Vec<ContentFile>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ContentFile {
    #[serde(rename(deserialize = "file"))]
    pub filename: PathBuf,
    #[serde(rename(deserialize = "hash"))]
    pub cid: ContentId,
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn scene_file_deserializes_from_json() {
    let json = include_str!("../fixtures/scene.json");
    let scene_file : SceneFile = serde_json::from_str(json).unwrap();
    assert_eq!(scene_file.kind, EntityType::Scene);
  }

  #[test]
  fn content_file_deserializes_from_json() {
    let json = "{\"file\":\"file.txt\",\"hash\":\"hash\"}";
    
    let content_file : ContentFile = serde_json::from_str(json).unwrap();
    
    let mut expected = PathBuf::new();
    expected.set_file_name("file.txt");
    assert_eq!(content_file.filename, expected);
    assert_eq!(content_file.cid, ContentId::new("hash"));
  }
}
