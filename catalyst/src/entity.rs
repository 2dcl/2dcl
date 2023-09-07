use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

use crate::{emote::Emote, profile::Profile, scene::Scene, wearable::Wearable, ContentId, HashId};

/// Represents an entity from the server (scene, wearable, profile)
///
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub version: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub kind: EntityType,
    pub pointers: Vec<String>,
    pub timestamp: u128,
    pub content: Vec<ContentFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContentFile {
    #[serde(rename(deserialize = "file", serialize = "file"))]
    pub filename: PathBuf,
    #[serde(rename(deserialize = "hash", serialize = "hash"))]
    pub cid: ContentId,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Metadata {
    #[serde(rename = "profile")]
    Profile(Profile),
    #[serde(rename = "scene")]
    Scene(Box<Scene>),
    #[serde(rename = "wearable")]
    Wearable(Wearable),
    #[serde(rename = "emote")]
    Emote(Emote),
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: EntityId(String::default()),
            version: Default::default(),
            kind: EntityType::Scene,
            pointers: Default::default(),
            timestamp: Default::default(),
            content: Default::default(),
            metadata: Default::default(),
        }
    }
}

impl Entity {
    /// Constructs a new `Entity` with an `EntityType` and an id as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::Entity;
    /// use catalyst::EntityType;
    /// let entity = Entity::new(EntityType::Scene, "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    ///
    /// assert_eq!(entity.kind, EntityType::Scene);
    /// assert_eq!(entity.id.hash(), "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    /// ```
    pub fn new<T>(kind: EntityType, id: T) -> Entity
    where
        T: AsRef<str>,
    {
        Entity {
            kind,
            id: EntityId::new(id),
            ..Default::default()
        }
    }

    /// Constructs a new `Profile` entity with id as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::Entity;
    /// use catalyst::EntityType;
    /// let entity = Entity::profile("bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    ///
    /// assert_eq!(entity.kind, EntityType::Profile);
    /// assert_eq!(entity.id.hash(), "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    /// ```
    pub fn profile<T>(id: T) -> Entity
    where
        T: AsRef<str>,
    {
        Entity::new(EntityType::Profile, id)
    }

    /// Constructs a new `Scene` entity with id as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::Entity;
    /// use catalyst::EntityType;
    /// let entity = Entity::scene("bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    ///
    /// assert_eq!(entity.kind, EntityType::Scene);
    /// assert_eq!(entity.id.hash(), "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    /// ```
    pub fn scene<T>(id: T) -> Entity
    where
        T: AsRef<str>,
    {
        Entity::new(EntityType::Scene, id)
    }

    /// Constructs a new `Wearable` entity with id as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::Entity;
    /// use catalyst::EntityType;
    /// let entity = Entity::wearable("bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    ///
    /// assert_eq!(entity.kind, EntityType::Wearable);
    /// assert_eq!(entity.id.hash(), "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    /// ```
    pub fn wearable<T>(id: T) -> Entity
    where
        T: AsRef<str>,
    {
        Entity::new(EntityType::Wearable, id)
    }
}

/// All available entity types
///
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub enum EntityType {
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "scene")]
    Scene,
    #[serde(rename = "wearable")]
    Wearable,
    #[serde(rename = "emote")]
    Emote,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialization = match self {
            EntityType::Profile => "profile",
            EntityType::Scene => "scene",
            EntityType::Wearable => "wearable",
            EntityType::Emote => "emote",
        };
        write!(f, "{}", serialization)
    }
}

/// Represents a hash that is used in the context of an entity id.
///
/// This struct implements `Display` to simplify the formatting of urls and messages.
///
/// ```
/// let entityId = catalyst::EntityId::new("a-missing-entity");
/// let message = format!("entity missing: {}", entityId);
/// assert_eq!(message, "entity missing: a-missing-entity");
/// ```
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct EntityId(pub HashId);

impl EntityId {
    /// Constructs a new entity id with id as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::EntityId;
    /// let id = EntityId::new("bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    ///
    /// assert_eq!(id.hash(), "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    /// ```
    pub fn new<T>(id: T) -> EntityId
    where
        T: AsRef<str>,
    {
        EntityId(id.as_ref().to_string())
    }

    /// Returns the hash for this id
    ///
    /// # Example
    ///
    /// ```
    /// use catalyst::EntityId;
    /// let id = EntityId::new("bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    ///
    /// assert_eq!(id.hash(), "bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki");
    /// ```
    pub fn hash(&self) -> &HashId {
        &self.0
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use dcl_common::Parcel;

    use crate::scene::SceneParcels;

    use super::*;

    #[test]
    fn entity_can_create_a_scene() {
        let scene = Entity::scene("id");
        assert_eq!(scene.kind, EntityType::Scene);
        assert_eq!(scene.id, EntityId::new("id"));
    }

    #[test]
    fn entity_can_create_a_profile() {
        let scene = Entity::profile("id");
        assert_eq!(scene.kind, EntityType::Profile);
        assert_eq!(scene.id, EntityId::new("id"));
    }

    #[test]
    fn entity_can_create_a_wearable() {
        let scene = Entity::wearable("id");
        assert_eq!(scene.kind, EntityType::Wearable);
        assert_eq!(scene.id, EntityId::new("id"));
    }

    #[test]
    fn entity_id_implements_display() {
        let id = EntityId::new("id");
        let id_string = format!("{}", id);
        assert_eq!(id_string, "id");
    }

    #[test]
    fn entity_id_implements_hash() {
        let id = EntityId::new("a-hash");
        assert_eq!(id.hash(), "a-hash");
    }

    #[test]
    fn entity_type_deserializes_correctly() {
        assert_eq!(
            EntityType::Profile,
            serde_json::from_str("\"profile\"").unwrap()
        );
        assert_eq!(
            EntityType::Scene,
            serde_json::from_str("\"scene\"").unwrap()
        );
        assert_eq!(
            EntityType::Wearable,
            serde_json::from_str("\"wearable\"").unwrap()
        );
    }

    #[test]
    fn entity_deserializes_correctly() {
        let response = include_str!("../fixtures/entity.json");
        let entity: Entity = serde_json::from_str(response).unwrap();
        let expected = Entity {
            id: EntityId("id".to_string()),
            version: "v3".to_string(),
            kind: EntityType::Scene,
            pointers: vec!["0,0".to_string()],
            timestamp: 1694091129392,
            content: vec![ContentFile {
                filename: PathBuf::from_str("a-file").unwrap(),
                cid: ContentId::new("a-cid"),
            }],
            metadata: Some(Metadata::Scene(Box::new(Scene {
                menu_bar_icon: None,
                is_portable_experience: None,
                main: None,
                scene: SceneParcels {
                    base: Parcel(0, 0),
                    parcels: vec![Parcel(0, 0)],
                },
                display: None,
                owner: None,
                contact: None,
                tags: None,
                source: None,
                spawn_points: None,
                required_permissions: None,
                feature_toggles: None,
                world_configuration: None,
                policy: None,
                allowed_media_hostnames: None,
                communications: None,
            }))),
        };

        assert_eq!(entity, expected);
    }
}
