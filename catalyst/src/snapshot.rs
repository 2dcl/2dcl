use crate::ContentId;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub hash: ContentId,
    pub time_range: TimeRange, // TODO(fran): use chrono?
    pub replaced_snapshot_hashes: Vec<ContentId>,
    pub number_of_entities: u32,
    pub generation_timestamp: u128,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    pub init_timestamp: u128,
    pub end_timestamp: u128,
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
            ContentId::new("bafybeibnnogb3wctyvaipalf6xdanhxyckx7fq4brfux2g4qc7qgfxndmu")
        );
    }
}
