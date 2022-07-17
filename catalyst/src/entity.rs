use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Entity {
  pub kind: EntityType,
  pub id: EntityId
}

impl Entity {

  pub fn new<T>(kind: EntityType, id: T) -> Entity
  where T: AsRef<str>
  {
    Entity {kind, id:EntityId::new(id)}
  }

  pub fn profile<T>(id: T) -> Entity
  where T: AsRef<str>
  {
    Entity::new(EntityType::Profile, id)
  }

  pub fn scene<T>(id: T) -> Entity
  where T: AsRef<str>
  {
    Entity::new(EntityType::Scene, id)
  }

  pub fn wearable<T>(id: T) -> Entity
  where T: AsRef<str>
  {
    Entity::new(EntityType::Wearable, id)
  }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum EntityType {
  #[serde(rename = "profile")]
  Profile,
  #[serde(rename = "scene")]
  Scene,
  #[serde(rename = "wearable")]
  Wearable
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EntityId(String);

impl EntityId {
  pub fn new<T>(id: T) -> EntityId
  where T: AsRef<str>
  {
    EntityId(id.as_ref().to_string())
  }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
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
  fn entity_type_deserializes_correctly() {
    assert_eq!(EntityType::Profile, serde_json::from_str("\"profile\"").unwrap());
    assert_eq!(EntityType::Scene, serde_json::from_str("\"scene\"").unwrap());
    assert_eq!(EntityType::Wearable, serde_json::from_str("\"wearable\"").unwrap());
  }
}
