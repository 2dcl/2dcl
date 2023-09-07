use crate::entity_information::AuthChain;
use crate::{ContentId, EntityId, EntityType};
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub hash: ContentId,
    pub last_included_deployment_timestamp: u128, // TODO(fran): use chrono?
    pub entities: EntitySnapshots,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct EntitySnapshots {
    pub scene: EntityTypeSnapshot,
    pub profile: EntityTypeSnapshot,
    pub wearable: EntityTypeSnapshot,
    pub store: EntityTypeSnapshot,
    pub emote: EntityTypeSnapshot,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EntityTypeSnapshot {
    pub hash: ContentId,
    pub last_included_deployment_timestamp: u128, // TODO(fran): use chrono?
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitySnapshot<T> {
    pub entity_id: EntityId,
    pub entity_type: EntityType,
    pub pointers: Vec<T>,
    pub local_timestamp: u128, // TODO(fran): use chrono?
    pub auth_chain: Vec<AuthChain>,
}

#[cfg(test)]
mod test {
    use crate::snapshot::Snapshot;
    use crate::ContentId;

    #[test]
    fn it_deserializes_from_json() {
        let json = include_str!("../fixtures/snapshot.json");
        let result: Snapshot = serde_json::from_str(json).unwrap();
        assert_eq!(
            result.hash,
            ContentId::new("bafybeifkmnczrywizlqhfirodivjenxyzu33k7wk4azxi7upklzfc5h3uy")
        );
    }
}
