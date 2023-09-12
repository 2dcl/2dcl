use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub type WearableId = String;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Wearable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_bar_icon: Option<String>,
    pub id: WearableId,
    pub name: Option<String>,
    pub description: String,
    pub i18n: Vec<I18n>,
    pub thumbnail: Option<String>,
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(flatten)]
    pub props: Option<Props>,
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
    pub merkle_proof: MerkleProof,
    pub content: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StandardProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<Rarity>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct WearableData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces: Option<Vec<HideableWearableCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hides: Option<Vec<HideableWearableCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
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
    #[serde(rename = "")]
    None,
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
            Rarity::None => write!(f, ""),
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

#[cfg(test)]
mod test {
    use crate::{Props, Rarity, StandardProps, Wearable, WearableCategory, WearableData};

    #[test]
    fn wearable_deserializes_correctly() {
        let response = include_str!("../fixtures/wearable.json");
        let wearable: Wearable = serde_json::from_str(response).unwrap();
        let expected = Wearable {
            menu_bar_icon: None,
            id: "id".to_string(),
            name: Some("name".to_string()),
            description: "description".to_string(),
            i18n: Vec::default(),
            thumbnail: Some("thumbnail.png".to_string()),
            image: Some("image.png".to_string()),
            metrics: None,
            props: Some(Props::Standard(StandardProps {
                collection_address: Some("address".to_string()),
                rarity: Some(Rarity::Common),
            })),
            data: WearableData {
                replaces: None,
                hides: None,
                tags: None,
                representations: Vec::default(),
                category: WearableCategory::LowerBody,
                removes_default_hiding: None,
            },
        };
        assert_eq!(wearable, expected);
    }
}
