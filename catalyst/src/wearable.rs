use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Wearable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_bar_icon: Option<String>,
    pub id: String,
    pub description: String,
    pub i18n: Vec<I18n>,
    pub thumbnail: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(flatten)]
    pub props: Props,
    pub data: WearableData,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Props {
    ThirdParty(ThirdPartyProps),
    Standard(StandardProps),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyProps {
    pub merkle_roof: MerkleProof,
    pub content: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StandardProps {
    pub collection_address: String,
    pub rarity: Rarity,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct WearableData {
    pub replaces: Vec<HideableWearableCategory>,
    pub hides: Vec<HideableWearableCategory>,
    pub tags: Vec<String>,
    pub representations: Vec<WearableRepresentation>,
    pub category: WearableCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removes_default_hiding: Option<Vec<HideableWearableCategory>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WearableRepresentation {
    pub body_shapes: Vec<BodyShape>,
    pub main_file: String,
    pub contents: Vec<String>,
    pub override_hides: Vec<HideableWearableCategory>,
    pub override_replaces: Vec<HideableWearableCategory>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum BodyShape {
    #[serde(rename(
        serialize = "urn:decentraland:off-chain:base-avatars:BaseMale",
        deserialize = "urn:decentraland:off-chain:base-avatars:BaseMale"
    ))]
    Male,
    #[serde(rename(
        serialize = "urn:decentraland:off-chain:base-avatars:BaseFemale",
        deserialize = "urn:decentraland:off-chain:base-avatars:BaseFemale"
    ))]
    Female,
}

impl BodyShape {
    pub fn get_urn(&self) -> &str {
        match self {
            BodyShape::Male => "urn:decentraland:off-chain:base-avatars:BaseMale",
            BodyShape::Female => "urn:decentraland:off-chain:base-avatars:BaseFemale",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum HideableWearableCategory {
    Wearable(WearableCategory),
    BodyPart(BodyPartCategory),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WearableCategory {
    Eyebrows,
    Eyes,
    FacialHair,
    Hair,
    BodyShape,
    Mouth,
    UpperBody,
    LowerBody,
    Feet,
    Earring,
    Eyewear,
    Hat,
    Helmet,
    Mask,
    Tiara,
    TopHead,
    Skin,
    HandsWear,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BodyPartCategory {
    Head,
    Hands,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProof {
    pub proof: Vec<String>,
    pub index: f32,
    pub hashing_keys: Vec<String>,
    pub entity_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    Unique,
    Mythic,
    Legendary,
    Epic,
    Rare,
    Uncommon,
    Common,
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rarity::Unique => write!(f, "unique"),
            Rarity::Mythic => write!(f, "mythic"),
            Rarity::Legendary => write!(f, "legendary"),
            Rarity::Epic => write!(f, "epic"),
            Rarity::Rare => write!(f, "rare"),
            Rarity::Uncommon => write!(f, "uncommon"),
            Rarity::Common => write!(f, "common"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct I18n {
    pub code: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Metrics {
    pub triangles: f32,
    pub materials: f32,
    pub textures: f32,
    pub meshes: f32,
    pub bodies: f32,
    pub entities: f32,
}
