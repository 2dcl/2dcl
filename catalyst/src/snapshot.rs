use dcl_common::Parcel;
use crate::{ContentId, Entity, EntityId, EntityType};
use crate::entity_information::AuthChain;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
  pub hash: ContentId,
  pub last_included_deployment_timestamp: u64, // TODO(fran): use chrono?
  pub entities: EntitySnapshots
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EntitySnapshots {
  pub scene: EntityTypeSnapshot,
  pub profile: EntityTypeSnapshot,
  pub wearable: EntityTypeSnapshot,
  pub store: EntityTypeSnapshot
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EntityTypeSnapshot {
  pub hash: ContentId,
  pub last_included_deployment_timestamp: u64, // TODO(fran): use chrono?
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitySnapshot<T> {
  pub entity_id: EntityId,
  pub entity_type: EntityType,
  pub pointers: Vec<T>,
  pub local_timestamp: u64, // TODO(fran): use chrono?
  pub auth_chain: Vec<AuthChain>
}

#[cfg(test)]
mod test {
  use crate::ContentId;
  use crate::Snapshot;

  #[test]
  fn it_deserializes_from_json() {
    let json = include_str!("../fixtures/snapshot.json");
    let result : Snapshot = serde_json::from_str(json).unwrap();
    assert_eq!(result.hash, ContentId::new("bafybeifkmnczrywizlqhfirodivjenxyzu33k7wk4azxi7upklzfc5h3uy"));
  }
}
