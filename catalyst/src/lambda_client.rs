use crate::*;
use serde::{Deserialize};

/// `LambdaClient` implements all the request to interact with [Catalyst Lambda](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas).
///
#[derive(Default)]
pub struct LambdaClient {}
impl LambdaClient {
  
  /// Implements [`/lambda/contracts/servers`](https://decentraland.github.io/catalyst-api-specs/#operation/getServers)
  pub async fn servers(server: &Server) -> Result<Vec<Server>>{
    let servers : Vec<Server> = server.get("/lambdas/contracts/servers").await?;
    Ok(servers)
  }
  
  /// Implements [`/lambda/status`](https://decentraland.github.io/catalyst-api-specs/#operation/getLambdaStatus)
  pub async fn status(server: &Server) -> Result<server::Status> {
    let status : server::Status = server.get("/lambdas/status").await?;
    Ok(status)
  }

  /// Implements [`/lambda/contentv1/scenes`](https://decentraland.github.io/catalyst-api-specs/#operation/getScenes)
  pub async fn scenes(server: &Server, start: &Parcel, end: &Parcel) -> Result<Vec<Scene>> {
    let scenes: ScenesResult = server.get(
      format!("/lambdas/contentv2/scenes?x1={}&y1={}&x2={}&y2={}", start.0, start.1, end.0, end.1)
    ).await?;
    Ok(scenes.data)
  }
}

#[derive(Debug, Deserialize)]
pub struct Scene {
  pub parcel_id: Parcel,
  pub root_cid: ContentId,
  pub scene_cid: ContentId
}

#[derive(Debug, Deserialize)]
struct ScenesResult {
  data: Vec<Scene>
}


#[cfg(test)]
mod tests {
  use super::*;
  use httpmock::prelude::*;

  #[test]
  fn it_can_be_created() {
    LambdaClient::default();
    assert!(true)
  }

  #[test]
  fn it_implements_server_list() {
    let response = "[
      {\"baseUrl\": \"https://server.com\",\"owner\": \"owner\",\"id\": \"id\"}
    ]";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.path("/lambdas/contracts/servers");
        then.status(200).body(response);
    });

    let server = Server::new(server.url(""));
    
    let servers = tokio_test::block_on(LambdaClient::servers(&server)).unwrap();
    
    m.assert();
    assert_eq!(servers[0].base_url, "https://server.com");
    assert_eq!(servers[0].owner, "owner");
    assert_eq!(servers[0].id, "id")
  }

  #[test]
  fn it_implements_server_status() {
    let response = "{\"version\": \"1.0\",\"currentTime\": 1628875330839,\"contentServerUrl\": \"https://content-server.com\",\"commitHash\": \"7890de4598f88a382863ea5f399b9cc17b80b42e\",\"catalystVersion\": \"1.3.3\"}";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.path("/lambdas/status");
        then.status(200).body(response);
    });

    let server = Server::new(server.url(""));

    let status = tokio_test::block_on(LambdaClient::status(&server)).unwrap();

    m.assert();

    assert_eq!(status.version, "1.0");
    assert_eq!(status.current_time, 1628875330839);
    assert_eq!(status.content_server_url, "https://content-server.com");
    assert_eq!(status.commit_hash, "7890de4598f88a382863ea5f399b9cc17b80b42e");
    assert_eq!(status.catalyst_version, "1.3.3");
  }

  #[test]
  fn it_implements_scenes() {
    let response = "{ 
      \"data\": [{
        \"parcel_id\": \"-9,-9\",
        \"root_cid\": \"QmaGgvj8EyWXFuyMs9GM7nrxzNSVFgByvu5PBniUfPYm6Q\",
        \"scene_cid\": \"QmQ2bvXj4DVsBM1m25YyM3quJLdApA1uaoDTe7LBJi9k2d\"
      }]
    }";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.path("/lambdas/contentv2/scenes");
        then.status(200).body(response);
    });

    let server = Server::new(server.url(""));

    let scenes = tokio_test::block_on(LambdaClient::scenes(
      &server,
      &Parcel(-1,-1),
      &Parcel( 1, 1)
    )).unwrap();

    m.assert();

    assert_eq!(scenes[0].parcel_id, Parcel(-9,-9));
    assert_eq!(scenes[0].root_cid, ContentId("QmaGgvj8EyWXFuyMs9GM7nrxzNSVFgByvu5PBniUfPYm6Q".to_string()));
    assert_eq!(scenes[0].scene_cid, ContentId("QmQ2bvXj4DVsBM1m25YyM3quJLdApA1uaoDTe7LBJi9k2d".to_string()));
  }

  
}
