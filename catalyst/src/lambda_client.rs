use crate::*;
use dcl_common::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CollectionsResponse {
    pub collections: Vec<CollectionData>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CollectionData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Erc721Entity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub image: String,
    pub thumbnail: String,
    pub attributes: Vec<Erc721Attribute>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Erc721Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    pub total_amount: u16,
    pub page_num: u16,
    pub page_size: u16,
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub urn: String,
    pub name: String,
    pub category: ItemCategory,
    pub rarity: Rarity,
    pub amount: u16,
    pub individual_data: Vec<IndividualData>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndividualData {
    pub id: String,
    pub token_id: String,
    pub transferred_at: String,
    pub price: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ItemCategory {
    Wearable(WearableCategory),
    Emote(EmoteCategory),
}

/// `LambdaClient` implements all the request to interact with [Catalyst Lambda](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas).
///
#[derive(Default)]
pub struct LambdaClient {}
impl LambdaClient {
    /// Implements [`/lambdas/status`](https://decentraland.github.io/catalyst-api-specs/#operation/getLambdaStatus)
    pub async fn status(server: &Server) -> Result<server::Status> {
        let status: server::Status = server.get("/lambdas/status").await?;
        Ok(status)
    }

    /// Implements [`/lambdas/collections`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getCollections)
    pub async fn collections(server: &Server) -> Result<CollectionsResponse> {
        let collections = server.get("/lambdas/collections").await?;
        Ok(collections)
    }

    /// Implements [`/lambdas/collections/contents/{urn}/thumbnail`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getThumbnail)
    pub async fn download_collection_thumbnail<V, T>(
        server: &Server,
        urn: T,
        filename: V,
    ) -> Result<()>
    where
        V: AsRef<Path>,
        T: AsRef<str>,
    {
        let response = server
            .raw_get(format!(
                "/lambdas/collections/contents/{}/thumbnail",
                urn.as_ref()
            ))
            .await?;
        if let Some(parent) = filename.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut dest = File::create(filename)?;
        let content = response.bytes().await?;
        dest.write_all(&content)?;
        Ok(())
    }

    /// Implements [`/lambdas/collections/contents/{urn}/image`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getImage)
    pub async fn download_collection_image<V, T>(server: &Server, urn: T, filename: V) -> Result<()>
    where
        V: AsRef<Path>,
        T: AsRef<str>,
    {
        let response = server
            .raw_get(format!(
                "/lambdas/collections/contents/{}/image",
                urn.as_ref()
            ))
            .await?;
        if let Some(parent) = filename.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut dest = File::create(filename)?;
        let content = response.bytes().await?;
        dest.write_all(&content)?;
        Ok(())
    }

    /// Implements [`/lambdas/collections/standard/erc721/{chainId}/{contract}/{option}/{emission}`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getStandardErc721)
    pub async fn erc721_entity<W, X, Y>(
        server: &Server,
        chain_id: W,
        contract_hash: X,
        token_identifier: Y,
        rarity: Option<Rarity>,
    ) -> Result<Erc721Entity>
    where
        W: AsRef<str>,
        X: AsRef<str>,
        Y: AsRef<str>,
    {
        let rarity = match rarity {
            Some(rarity) => format!("{}", rarity),
            None => String::default(),
        };

        let collections = server
            .get(format!(
                "/lambdas/collections/standard/erc721/{}/{}/{}/{}",
                chain_id.as_ref(),
                contract_hash.as_ref(),
                token_identifier.as_ref(),
                rarity
            ))
            .await?;
        Ok(collections)
    }

    /// Implements [`/lambdas/users/{address}/wearables`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getWearables)
    pub async fn wearables_for_address<T>(server: &Server, address: T) -> Result<Items>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!("/lambdas/users/{}/wearables", address.as_ref()))
            .await?;
        Ok(response)
    }

    /// Implements [`/lambdas/users/{address}/emotes`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getEmotes)
    pub async fn emotes_for_address<T>(server: &Server, address: T) -> Result<Items>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!("/lambdas/users/{}/emotes", address.as_ref()))
            .await?;
        Ok(response)
    }

    /*    /// Implements [`/lambda/contracts/servers`](https://decentraland.github.io/catalyst-api-specs/#operation/getServers)
    pub async fn servers(server: &Server) -> Result<Vec<Server>> {
        let servers: Vec<Server> = server.get("/lambdas/contracts/servers").await?;
        Ok(servers)
    }

    /// Implements [`/lambda/contentv1/scenes`](https://decentraland.github.io/catalyst-api-specs/#operation/getScenes)
    pub async fn scenes(server: &Server, start: &Parcel, end: &Parcel) -> Result<Vec<Scene>> {
        let scenes: ScenesResult = server
            .get(format!(
                "/lambdas/contentv2/scenes?x1={}&y1={}&x2={}&y2={}",
                start.0, start.1, end.0, end.1
            ))
            .await?;
        Ok(scenes.data)
    }

    pub async fn scene_descriptor<T>(server: &Server, scene: Entity<T>) -> Result<SceneDescriptor>
    where T : AsRef<str>
    {
        Ok(SceneDescriptor {})
    } */
}

// #[derive(Debug, Deserialize)]
// pub struct SceneDescriptor {

// }

// #[derive(Debug, Deserialize)]
// pub struct Scene {
//     pub parcel_id: Parcel,
//     pub root_cid: ContentId,
//     pub scene_cid: ContentId,
// }

// #[derive(Debug, Deserialize)]
// struct ScenesResult {
//     data: Vec<Scene>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use tempdir::TempDir;

    #[test]
    fn it_can_be_created() {
        LambdaClient::default();
        assert!(true)
    }

    #[test]
    fn it_implements_server_status() {
        let response = include_str!("../fixtures/status.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/status");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdaClient::status(&server)).unwrap();

        m.assert();

        let expected: server::Status = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_collections() {
        let response = include_str!("../fixtures/collections.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/collections");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdaClient::collections(&server)).unwrap();

        m.assert();

        let expected: CollectionsResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_downloads_collections_thumbnail() {
        let response = "Collection thumbnail";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/collections/contents/a-urn/thumbnail");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let tmp_dir = TempDir::new("lambda-client-test").unwrap();
        let filename = tmp_dir.path().join("thumbnail.png");

        tokio_test::block_on(LambdaClient::download_collection_thumbnail(
            &server,
            &"a-urn".to_string(),
            filename.clone(),
        ))
        .unwrap();

        m.assert();

        assert_eq!(
            std::fs::read_to_string(filename).unwrap(),
            "Collection thumbnail"
        );
    }

    #[test]
    fn it_downloads_collections_image() {
        let response = "Collection image";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/collections/contents/a-urn/image");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let tmp_dir = TempDir::new("lambda-client-test").unwrap();
        let filename = tmp_dir.path().join("image.png");

        tokio_test::block_on(LambdaClient::download_collection_image(
            &server,
            &"a-urn".to_string(),
            filename.clone(),
        ))
        .unwrap();

        m.assert();

        assert_eq!(
            std::fs::read_to_string(filename).unwrap(),
            "Collection image"
        );
    }

    #[test]
    fn it_gets_erc721_entities() {
        let response = include_str!("../fixtures/erc721_entity.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path(
                "/lambdas/collections/standard/erc721/chain_id/contract_hash/token_identifier/",
            );
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdaClient::erc721_entity(
            &server,
            "chain_id",
            "contract_hash",
            "token_identifier",
            None,
        ))
        .unwrap();

        m.assert();

        let expected: Erc721Entity = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_wearables_for_address() {
        let response = include_str!("../fixtures/wearables_for_address.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/users/an-address/wearables");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result =
            tokio_test::block_on(LambdaClient::wearables_for_address(&server, "an-address"))
                .unwrap();

        m.assert();

        let expected: Items = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_emotes_for_address() {
        let response = include_str!("../fixtures/emotes_for_address.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/users/an-address/emotes");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result =
            tokio_test::block_on(LambdaClient::emotes_for_address(&server, "an-address")).unwrap();

        m.assert();

        let expected: Items = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }
    /*

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
     fn it_implements_scene_descriptor() {
         let response = include_str!("../fixtures/genesis_plaza_scene.json");

         let server = MockServer::start();

         let m = server.mock(|when, then| {
             when.path("/lambdas/contentv2/contents/hash");
             then.status(200).body(response);
         });

         let server = Server::new(server.url(""));

         let response = tokio_test::block_on(LambdaClient::scene_descriptor(
             &server,
             Entity::scene("hash")
         )).unwrap();

         m.assert();

         let expected : SceneDescriptor = serde_json::from_str(response).unwrap();
         assert_eq!(content_file, expected);

     } */
}
