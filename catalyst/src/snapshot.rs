use crate::ContentId;
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
  pub scene: EntitySnapshot,
  pub profile: EntitySnapshot,
  pub wearable: EntitySnapshot,
  pub store: EntitySnapshot
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EntitySnapshot {
  pub hash: ContentId,
  pub last_included_deployment_timestamp: u64, // TODO(fran): use chrono?
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
