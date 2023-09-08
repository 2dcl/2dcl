use crate::Server;
use dcl_common::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AboutResponse {
    pub healthy: bool,
    pub configurations: Configurations,
    pub accepting_users: bool,
    pub content: ContentStatus,
    pub lambdas: LambdasStatus,
    pub comms: CommsStatus,
    pub bff: Option<BffStatus>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentStatus {
    pub healthy: bool,
    pub version: Option<String>,
    pub syncronization_status: Option<String>,
    pub commit_hash: Option<String>,
    pub public_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LambdasStatus {
    pub healthy: bool,
    pub version: Option<String>,
    pub commit_hash: Option<String>,
    pub public_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommsStatus {
    pub healthy: bool,
    pub protocol: Option<String>,
    pub version: Option<String>,
    pub commit_hash: Option<String>,
    pub users_count: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BffStatus {
    pub healthy: bool,
    pub user_count: Option<u64>,
    pub protocol_version: Option<String>,
    pub public_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Configurations {
    pub network_id: Option<f32>,
    pub global_scenes_urn: Option<Vec<String>>,
    pub scenes_urn: Option<Vec<String>>,
    pub realm_name: String,
}

/// Implements all the request to interact with the whole catalyst or shared services [Global](https://decentraland.github.io/catalyst-api-specs/#tag/Global).
///
#[derive(Default)]
pub struct GlobalClient {}

impl GlobalClient {
    /// Returns detailed information about the services health and its configuration
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Global)
    pub async fn about(server: &Server) -> Result<AboutResponse> {
        let result = server.get("/about").await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn it_implements_about() {
        let response = include_str!("../fixtures/about.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/about");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: AboutResponse = tokio_test::block_on(GlobalClient::about(&server)).unwrap();

        m.assert();

        let expected: AboutResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }
}
