use dcl_common::EthNetwork;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentServerStatus {
    #[serde(skip)]
    pub name: String,
    pub version: String,
    #[serde(skip)]
    pub current_time: u64, // TODO(fran): use chrono?
    #[serde(skip)]
    pub last_immutable_time: u64, // TODO(fran): use chrono?
    #[serde(skip)]
    pub history_size: u64,
    pub synchronization_status: SynchronizationStatus,
    pub commit_hash: String,
    pub catalyst_version: String,
    pub eth_network: EthNetwork,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct SynchronizationStatus {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn it_deserializes_content_status_from_json() {
        let json = include_str!("../fixtures/content_server_status.json");
        let result: ContentServerStatus = serde_json::from_str(json).unwrap();
        assert_eq!(result.version, "v3");
    }
}
