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
    #[serde(rename(
        deserialize = "isPortableExperience",
        serialize = "isPortableExperience"
    ))]
    pub is_portable_experience: Option<bool>,
    pub main: Option<String>,
    pub scene: SceneParcels,
    pub display: Option<Display>,
    pub owner: Option<String>,
    pub contact: Option<Contact>,
    pub tags: Option<Vec<String>>,
    pub source: Option<Source>,
    #[serde(rename(deserialize = "spawnPoints", serialize = "spawnPoints"))]
    pub spawn_points: Option<Vec<SpawnPoints>>,
    #[serde(rename(deserialize = "requiredPermissions", serialize = "requiredPermissions"))]
    pub required_permissions: Option<Vec<RequiredPermission>>,
    #[serde(rename(deserialize = "featureToggles", serialize = "featureToggles"))]
    pub feature_toggles: Option<FeatureToggles>,
    #[serde(rename(deserialize = "worldConfiguration", serialize = "worldConfiguration"))]
    pub world_configuration: Option<WorldConfiguration>,
    pub policy: Option<Policy>,
    #[serde(rename(
        deserialize = "allowedMediaHostnames",
        serialize = "allowedMediaHostnames"
    ))]
    pub allowed_media_hostnames: Option<Vec<String>>,
    pub communications: Option<Communications>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum RequiredPermission {
    #[serde(rename(
        deserialize = "ALLOW_MEDIA_HOSTNAMES",
        serialize = "ALLOW_MEDIA_HOSTNAMES"
    ))]
    AllowMediaHostnames,
    #[serde(rename(
        deserialize = "ALLOW_TO_MOVE_PLAYER_INSIDE_SCENE",
        serialize = "ALLOW_TO_MOVE_PLAYER_INSIDE_SCENE"
    ))]
    AllowToMovePlayerInsideScene,
    #[serde(rename(
        deserialize = "ALLOW_TO_TRIGGER_AVATAR_EMOTE",
        serialize = "ALLOW_TO_TRIGGER_AVATAR_EMOTE"
    ))]
    AllowToTriggerAvatarEmote,
    #[serde(rename(deserialize = "OPEN_EXTERNAL_LINK", serialize = "OPEN_EXTERNAL_LINK"))]
    OpenExternalLink,
    #[serde(rename(deserialize = "USE_FETCH", serialize = "USE_FETCH"))]
    UseFetch,
    #[serde(rename(deserialize = "USE_WEB3_API", serialize = "USE_WEB3_API"))]
    UseWeb3Api,
    #[serde(rename(deserialize = "USE_WEBSOCKET", serialize = "USE_WEBSOCKET"))]
    UseWebsocket,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct WorldConfiguration {
    pub name: Option<String>,
    pub skybox: Option<i32>,
    #[serde(rename(deserialize = "minimapVisible", serialize = "minimapVisible"))]
    pub minimap_visible: Option<bool>,
    #[serde(rename(deserialize = "miniMapConfig", serialize = "miniMapConfig"))]
    pub mini_map_config: Option<MiniMapConfig>,
    #[serde(rename(deserialize = "skyboxConfig", serialize = "skyboxConfig"))]
    pub sky_box_config: Option<SkyBoxConfig>,
    #[serde(rename(deserialize = "fixedAdapter", serialize = "fixedAdapter"))]
    pub fixed_adapter: Option<String>,
    #[serde(rename(deserialize = "placesConfig", serialize = "placesConfig"))]
    pub places_config: Option<PlacesConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct PlacesConfig {
    #[serde(rename(deserialize = "optOut", serialize = "optOut"))]
    opt_out: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SkyBoxConfig {
    pub fixed_time: Option<i32>,
    pub textures: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct MiniMapConfig {
    pub visible: Option<bool>,
    #[serde(rename(deserialize = "dataImage", serialize = "dataImage"))]
    pub data_image: Option<String>,
    #[serde(rename(deserialize = "estateImage", serialize = "estateImage"))]
    pub estate_image: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct FeatureToggles {
    #[serde(rename(deserialize = "voiceChat", serialize = "voiceChat"))]
    pub voice_chat: Option<Toggle>,
    #[serde(rename(deserialize = "portableExperiences", serialize = "portableExperiences"))]
    pub portable_experiences: Option<Toggle>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Toggle {
    #[serde(rename(deserialize = "enabled", serialize = "enabled"))]
    Enabled,
    #[serde(rename(deserialize = "disabled", serialize = "disabled"))]
    Disabled,
    #[serde(rename(deserialize = "hideUi", serialize = "hideUi"))]
    HideUi,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Source {
    pub version: Option<i32>,
    pub origin: String,
    #[serde(rename(deserialize = "projectId", serialize = "projectId"))]
    pub project_id: String,
    pub point: Option<Point>,
    pub rotation: Option<Rotation>,
    pub layout: Option<Layout>,
    #[serde(rename(deserialize = "isEmpty", serialize = "isEmpty"))]
    pub is_empty: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Layout {
    pub rows: i32,
    pub cols: i32,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub enum Rotation {
    #[default]
    #[serde(rename(deserialize = "north", serialize = "north"))]
    North,
    #[serde(rename(deserialize = "east", serialize = "east"))]
    East,
    #[serde(rename(deserialize = "south", serialize = "south"))]
    South,
    #[serde(rename(deserialize = "west", serialize = "west"))]
    West,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SpawnPoints {
    pub name: Option<String>,
    pub position: SpawnPosition,
    pub default: Option<bool>,
    #[serde(rename(deserialize = "cameraTarget", serialize = "cameraTarget"))]
    pub camera_target: Option<SinglePosition>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum SpawnPosition {
    SinglePosition(SinglePosition),
    MultiPosition(MultiPosition),
}

impl Default for SpawnPosition {
    fn default() -> Self {
        SpawnPosition::SinglePosition(SinglePosition::default())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SinglePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct MultiPosition {
    pub x: Vec<i32>,
    pub y: Vec<i32>,
    pub z: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Display {
    pub title: Option<String>,
    pub description: Option<String>,
    pub navmap_thumbnail: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub im: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SceneParcels {
    pub base: Parcel,
    pub parcels: Vec<Parcel>,
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
