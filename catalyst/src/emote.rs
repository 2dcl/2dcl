use serde::Deserialize;
use serde::Serialize;

use crate::wearable::BodyShape;
use crate::wearable::I18n;
use crate::wearable::Metrics;
use crate::wearable::Props;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_bar_icon: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub i18n: Vec<I18n>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(flatten)]
    pub props: Option<Props>,
    #[serde(rename = "emoteDataADR74")]
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

#[cfg(test)]
mod test {
    use crate::{
        emote::{Emote, EmoteCategory, EmoteDataADR74},
        wearable::{Props, Rarity, StandardProps},
    };

    #[test]
    fn emote_deserializes_correctly() {
        let response = include_str!("../fixtures/emote.json");
        let emote: Emote = serde_json::from_str(response).unwrap();
        let expected = Emote {
            menu_bar_icon: None,
            id: "id".to_string(),
            name: Some("name".to_string()),
            description: Some("description".to_string()),
            i18n: Vec::default(),
            thumbnail: Some("thumbnail.png".to_string()),
            image: Some("image.png".to_string()),
            metrics: None,
            props: Some(Props::Standard(StandardProps {
                collection_address: Some("address".to_string()),
                rarity: Some(Rarity::Common),
            })),
            emote_data_adr74: EmoteDataADR74 {
                category: EmoteCategory::Dance,
                representations: Vec::default(),
                tags: Vec::default(),
                it_loops: true,
            },
        };
        assert_eq!(emote, expected);
    }
}
