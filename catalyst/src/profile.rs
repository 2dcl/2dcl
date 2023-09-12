use serde::Deserialize;
use serde::Serialize;

use crate::wearable::WearableCategory;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Profile {
    pub avatars: Vec<Avatar>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub user_id: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual_orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hobbies: Option<String>,
    pub eth_address: String,
    pub version: f32,
    pub tutorial_step: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>,
    pub has_claimed_name: bool,
    pub avatar: AvatarInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Link {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarInfo {
    pub body_shape: String,
    pub eyes: ColoredAvatarPart,
    pub hair: ColoredAvatarPart,
    pub skin: ColoredAvatarPart,
    pub wearables: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_render: Option<Vec<WearableCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotes: Option<Vec<EmoteData>>,
    pub snapshots: SnapshotsData,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ColoredAvatarPart {
    pub color: Color,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f32>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EmoteData {
    pub slot: f32,
    pub urn: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SnapshotsData {
    pub face256: String,
    pub body: String,
}

#[cfg(test)]
mod test {
    use crate::profile::{Avatar, AvatarInfo, Color, ColoredAvatarPart, Profile, SnapshotsData};

    #[test]
    fn profile_deserializes_correctly() {
        let response = include_str!("../fixtures/profile.json");
        let profile: Profile = serde_json::from_str(response).unwrap();
        let expected = Profile {
            avatars: vec![Avatar {
                user_id: "id".to_string(),
                name: "name".to_string(),
                description: "description".to_string(),
                links: None,
                country: None,
                employment_status: None,
                gender: None,
                pronouns: None,
                relationship_status: None,
                sexual_orientation: None,
                language: None,
                profession: None,
                birthdate: None,
                real_name: None,
                hobbies: None,
                eth_address: "address".to_string(),
                version: 1.,
                tutorial_step: 1.,
                email: None,
                blocked: None,
                muted: None,
                interests: None,
                has_claimed_name: true,
                avatar: AvatarInfo {
                    body_shape: "body_shape".to_string(),
                    eyes: ColoredAvatarPart {
                        color: Color {
                            r: 1.,
                            g: 1.,
                            b: 1.,
                            a: Some(1.),
                        },
                    },
                    hair: ColoredAvatarPart {
                        color: Color {
                            r: 1.,
                            g: 1.,
                            b: 1.,
                            a: Some(1.),
                        },
                    },
                    skin: ColoredAvatarPart {
                        color: Color {
                            r: 1.,
                            g: 1.,
                            b: 1.,
                            a: Some(1.),
                        },
                    },
                    wearables: Vec::default(),
                    force_render: None,
                    emotes: None,
                    snapshots: SnapshotsData {
                        face256: "face".to_string(),
                        body: "body".to_string(),
                    },
                },
            }],
        };
        assert_eq!(profile, expected);
    }
}
