use crate::*;

use dcl_common::Result;

use std::fs::File;

use std::io::Write;
use std::path::Path;

/// `ContentClient` implements all the request to interact with [Catalyst Content Servers](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server).
///
#[derive(Default)]
pub struct ContentClient {}

impl ContentClient {
    pub async fn download<U, V>(server: &Server, hash_id: U, filename: V) -> Result<()>
    where
        U: AsRef<str> + std::fmt::Display,
        V: AsRef<Path>,
    {
        let response = server
            .raw_get(format!("/content/contents/{}", hash_id))
            .await?;
        let mut dest = File::create(filename)?;
        let content = response.bytes().await?;
        dest.write_all(&content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dcl_common::*;
    use httpmock::prelude::*;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn it_implements_active_entities() {
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
            &Parcel(-1, -1),
            &Parcel(1, 1),
        ))
        .unwrap();

        m.assert();

        assert_eq!(scenes[0].parcel_id, Parcel(-9, -9));
        assert_eq!(
            scenes[0].root_cid,
            ContentId("QmaGgvj8EyWXFuyMs9GM7nrxzNSVFgByvu5PBniUfPYm6Q".to_string())
        );
        assert_eq!(
            scenes[0].scene_cid,
            ContentId("QmQ2bvXj4DVsBM1m25YyM3quJLdApA1uaoDTe7LBJi9k2d".to_string())
        );
    }

    #[test]
    fn it_downloads_file() {
        let response = "File Content";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/content/contents/a-hash");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let tmp_dir = TempDir::new("content-client-test").unwrap();
        let filename = tmp_dir.path().join("test.txt");

        tokio_test::block_on(ContentClient::download(&server, "a-hash", filename.clone())).unwrap();

        m.assert();

        assert_eq!(fs::read_to_string(filename).unwrap(), "File Content");
    }
}
