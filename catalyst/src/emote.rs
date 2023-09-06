use serde::Deserialize;
use serde::Serialize;

use crate::wearable::BodyShape;
use crate::wearable::I18n;
use crate::wearable::Metrics;
use crate::wearable::StandardProps;
use crate::wearable::ThirdPartyProps;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_bar_icon: Option<String>,
    pub id: String,
    pub name: String,
    pub description: String,
    pub i18n: Vec<I18n>,
    pub thumbnail: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_props: Option<ThirdPartyProps>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_props: Option<StandardProps>,

    pub emote_data_adr74: EmoteDataADR74,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EmoteDataADR74 {
    pub category: EmoteCategory,
    pub representations: Vec<EmoteRepresentationADR74>,
    pub tags: Vec<String>,
    #[serde(rename(deserialize = "loop", serialize = "loop"))]
    pub it_loops: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmoteRepresentationADR74 {
    pub body_shapes: Vec<BodyShape>,
    pub main_file: String,
    pub contents: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EmoteCategory {
    Dance,
    Stunt,
    Greetings,
    Fun,
    Poses,
    Reactions,
    Horror,
    Miscellaneuos,
}
