use dcl_common::Parcel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Scene {
    #[serde(rename(deserialize = "menuBarIcon", serialize = "menuBarIcon"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_bar_icon: Option<String>,
    #[serde(rename(
        deserialize = "isPortableExperience",
        serialize = "isPortableExperience"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_portable_experience: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    pub scene: SceneParcels,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "spawnPoints", serialize = "spawnPoints"))]
    pub spawn_points: Option<Vec<SpawnPoints>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "requiredPermissions", serialize = "requiredPermissions"))]
    pub required_permissions: Option<Vec<RequiredPermission>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "featureToggles", serialize = "featureToggles"))]
    pub feature_toggles: Option<FeatureToggles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "worldConfiguration", serialize = "worldConfiguration"))]
    pub world_configuration: Option<WorldConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
    #[serde(rename(
        deserialize = "allowedMediaHostnames",
        serialize = "allowedMediaHostnames"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_media_hostnames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications: Option<Communications>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct WorldConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skybox: Option<f32>,
    #[serde(rename(deserialize = "minimapVisible", serialize = "minimapVisible"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimap_visible: Option<bool>,
    #[serde(rename(deserialize = "miniMapConfig", serialize = "miniMapConfig"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_map_config: Option<MiniMapConfig>,
    #[serde(rename(deserialize = "skyboxConfig", serialize = "skyboxConfig"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_box_config: Option<SkyBoxConfig>,
    #[serde(rename(deserialize = "fixedAdapter", serialize = "fixedAdapter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_adapter: Option<String>,
    #[serde(rename(deserialize = "placesConfig", serialize = "placesConfig"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub places_config: Option<PlacesConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct PlacesConfig {
    #[serde(rename(deserialize = "optOut", serialize = "optOut"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_out: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct SkyBoxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct MiniMapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename(deserialize = "dataImage", serialize = "dataImage"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_image: Option<String>,
    #[serde(rename(deserialize = "estateImage", serialize = "estateImage"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estate_image: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct FeatureToggles {
    #[serde(rename(deserialize = "voiceChat", serialize = "voiceChat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat: Option<Toggle>,
    #[serde(rename(deserialize = "portableExperiences", serialize = "portableExperiences"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portable_experiences: Option<Toggle>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Toggle {
    #[serde(rename(deserialize = "enabled", serialize = "enabled"))]
    Enabled,
    #[serde(rename(deserialize = "disabled", serialize = "disabled"))]
    Disabled,
    #[serde(rename(deserialize = "hideUi", serialize = "hideUi"))]
    HideUi,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<f32>,
    pub origin: String,
    #[serde(rename(deserialize = "projectId", serialize = "projectId"))]
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Rotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(rename(deserialize = "isEmpty", serialize = "isEmpty"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Layout {
    pub rows: i32,
    pub cols: i32,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PointType {
    Point(Point),
    Str(String),
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct SpawnPoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub position: SpawnPosition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename(deserialize = "cameraTarget", serialize = "cameraTarget"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_target: Option<SinglePosition>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SpawnPosition {
    SinglePosition(SinglePosition),
    MultiPosition(MultiPosition),
}

impl Default for SpawnPosition {
    fn default() -> Self {
        SpawnPosition::SinglePosition(SinglePosition::default())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct SinglePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct MultiPosition {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Display {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "navmapThumbnail", serialize = "navmapThumbnail"))]
    pub navmap_thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub im: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SceneParcels {
    pub base: Parcel,
    pub parcels: Vec<Parcel>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Communications {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signalling: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
