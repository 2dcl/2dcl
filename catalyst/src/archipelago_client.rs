use crate::{
    island::{Island, IslandId, Peer},
    Server,
};
use dcl_common::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct IslandsList {
    pub ok: bool,
    pub islands: Vec<Island>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PeersList {
    pub ok: bool,
    pub peers: Vec<Peer>,
}

/// Implements all the request to interact with [Catalyst Archipelago Servers](https://decentraland.github.io/catalyst-api-specs/#tag/Archipelago).
///
#[derive(Default)]
pub struct ArchipelagoClient {}

impl ArchipelagoClient {
    /// Returns a list of communication islands in the server with details about the peers present in each island.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Archipelago/operation/getIslands)
    pub async fn get_islands_list(server: &Server) -> Result<IslandsList> {
        let result = server.get("/comms/islands").await?;
        Ok(result)
    }

    /// Returns a list of peers in the server with details about their positions and islands.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Archipelago/operation/getPeers)
    pub async fn get_peers_list(server: &Server) -> Result<PeersList> {
        let result = server.get("/comms/peers").await?;
        Ok(result)
    }

    /// Returns the information about the island with the provided `island_id`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Archipelago/operation/getIsland)
    pub async fn get_island(server: &Server, island_id: &IslandId) -> Result<Island> {
        let result = server.get(format!("/comms/islands/{}", island_id)).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn it_gets_islands_list() {
        let response = include_str!("../fixtures/islands_list.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/comms/islands");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: IslandsList =
            tokio_test::block_on(ArchipelagoClient::get_islands_list(&server)).unwrap();

        m.assert();

        let expected: IslandsList = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_island() {
        let response = include_str!("../fixtures/island.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/comms/islands/id");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: Island =
            tokio_test::block_on(ArchipelagoClient::get_island(&server, &"id".to_string()))
                .unwrap();

        m.assert();

        let expected: Island = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_peers_list() {
        let response = include_str!("../fixtures/peers_list.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/comms/peers");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: PeersList =
            tokio_test::block_on(ArchipelagoClient::get_peers_list(&server)).unwrap();

        m.assert();

        let expected: PeersList = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }
}
