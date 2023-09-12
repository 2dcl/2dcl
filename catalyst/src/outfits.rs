use serde::Deserialize;
use serde::Serialize;

use crate::BodyShape;
use crate::ColoredAvatarPart;
use crate::WearableCategory;
use crate::WearableId;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Outfits {
    pub outfits: Vec<OutfitSlot>,
    pub names_for_extra_slots: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OutfitSlot {
    pub slot: i32,
    pub outfit: Outfit,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Outfit {
    pub body_shape: BodyShape,
    pub eyes: ColoredAvatarPart,
    pub hair: ColoredAvatarPart,
    pub skin: ColoredAvatarPart,
    pub wearables: Vec<WearableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_render: Option<Vec<WearableCategory>>,
}

#[cfg(test)]
mod test {

    use crate::{BodyShape, Color, ColoredAvatarPart, Outfit, OutfitSlot, Outfits};

    #[test]
    fn outfits_deserializes_correctly() {
        let response: &str = include_str!("../fixtures/outfits.json");
        let outfits: Outfits = serde_json::from_str(response).unwrap();
        let expected = Outfits {
            outfits: vec![OutfitSlot {
                outfit: Outfit {
                    body_shape: BodyShape::Female,
                    eyes: ColoredAvatarPart {
                        color: Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: Some(0.),
                        },
                    },
                    hair: ColoredAvatarPart {
                        color: Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: Some(0.),
                        },
                    },
                    skin: ColoredAvatarPart {
                        color: Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: Some(0.),
                        },
                    },
                    wearables: Vec::default(),
                    force_render: None,
                },
                slot: 0,
            }],
            names_for_extra_slots: Vec::default(),
        };

        assert_eq!(outfits, expected);
    }
}
