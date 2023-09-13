use crate::*;
use dcl_common::Result;
use serde::{Deserialize, Serialize};
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
pub struct ItemsResponse {
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
    pub rarity: Option<Rarity>,
    pub amount: u16,
    pub individual_data: Vec<IndividualData>,
    pub entity: Option<Entity>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndividualData {
    pub id: String,
    pub token_id: Option<String>,
    pub transferred_at: Option<String>,
    pub price: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ItemCategory {
    Wearable(WearableCategory),
    Emote(EmoteCategory),
    ThirdParty(String),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NamesResponse {
    pub total_amount: u16,
    pub page_num: u16,
    pub page_size: u16,
    pub elements: Vec<Name>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub name: String,
    pub contract_address: String,
    pub token_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LandsResponse {
    pub total_amount: u16,
    pub page_num: u16,
    pub page_size: u16,
    pub elements: Vec<Land>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Land {
    pub name: String,
    pub contract_address: String,
    pub token_id: String,
    pub category: String,
    pub description: String,
    pub price: String,
    pub image: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WearableSortingOrder {
    TransferredAt,
    Rarity,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HotScene {
    pub id: String,
    pub name: String,
    pub base_coords: Parcel,
    pub users_total_count: u32,
    pub parcels: Vec<Parcel>,
    pub thumbnail: Option<String>,
    pub creator: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<String>,
}

type Parcel = [i32; 2];

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Realm {
    pub server_name: String,
    pub url: String,
    pub user_parcels: Vec<Parcel>,
    pub users_count: u32,
    pub layer: Option<String>,
    pub thumbnail: Option<String>,
    pub max_users: Option<u32>,
}

impl std::fmt::Display for WearableSortingOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialization = match self {
            WearableSortingOrder::TransferredAt => "transferredAt",
            WearableSortingOrder::Rarity => "rarity",
        };
        write!(f, "{}", serialization)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct WearablesSearchParameters {
    include_entities: Option<bool>,
    include_third_party: Option<bool>,
    page_parameters: PageParameters,
    order_by: Option<WearableSortingOrder>,
}

impl WearablesSearchParameters {
    pub fn to_query_string(&self) -> String {
        let mut query_string = "?".to_string();
        if let Some(include_entities) = &self.include_entities {
            query_string += &format!("includeEntities={}&", include_entities);
        }
        if let Some(include_third_party) = &self.include_third_party {
            query_string += &format!("include_third_party={}&", include_third_party);
        }
        if let Some(page_num) = &self.page_parameters.page_num {
            query_string += &format!("pageNum={}&", page_num);
        }
        if let Some(page_size) = &self.page_parameters.page_size {
            query_string += &format!("pageSize={}&", page_size);
        }
        if let Some(order_by) = &self.order_by {
            query_string += &format!("orderBy={}", order_by);
        }
        query_string
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct EmotesSearchParameters {
    collection_id: Option<String>,
    include_entities: Option<bool>,
    page_parameters: PageParameters,
}

impl EmotesSearchParameters {
    pub fn to_query_string(&self) -> String {
        let mut query_string = "?".to_string();
        if let Some(include_entities) = &self.include_entities {
            query_string += &format!("includeEntities={}&", include_entities);
        }
        if let Some(collection_id) = &self.collection_id {
            query_string += &format!("collectionId={}&", collection_id);
        }
        if let Some(page_num) = &self.page_parameters.page_num {
            query_string += &format!("pageNum={}&", page_num);
        }
        if let Some(page_size) = &self.page_parameters.page_size {
            query_string += &format!("pageSize={}", page_size);
        }
        query_string
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct PageParameters {
    page_num: Option<u16>,
    page_size: Option<u16>,
}

impl PageParameters {
    pub fn to_query_string(&self) -> String {
        let mut query_string = "?".to_string();
        if let Some(page_num) = &self.page_num {
            query_string += &format!("pageNum={}&", page_num);
        }
        if let Some(page_size) = &self.page_size {
            query_string += &format!("pageSize={}", page_size);
        }
        query_string
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ThirdPartyIntegrations {
    pub data: Vec<ThirdPartyIntegration>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ThirdPartyIntegration {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub urn: Urn,
}

/// `LambdasClient` implements all the request to interact with [Catalyst Lambda](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas).
///
#[derive(Default)]
pub struct LambdasClient {}
impl LambdasClient {
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
    pub async fn wearables_for_address<T>(
        server: &Server,
        address: T,
        parameters: WearablesSearchParameters,
    ) -> Result<ItemsResponse>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!(
                "/lambdas/users/{}/wearables{}",
                address.as_ref(),
                parameters.to_query_string()
            ))
            .await?;
        Ok(response)
    }

    /// Implements [`/lambdas/users/{address}/emotes`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getEmotes)
    pub async fn emotes_for_address<T>(
        server: &Server,
        address: T,
        parameters: EmotesSearchParameters,
    ) -> Result<ItemsResponse>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!(
                "/lambdas/users/{}/emotes{}",
                address.as_ref(),
                parameters.to_query_string()
            ))
            .await?;
        Ok(response)
    }

    /// Implements [`/lambdas/users/{address}/names`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getNames)
    pub async fn names_for_address<T>(
        server: &Server,
        address: T,
        parameters: PageParameters,
    ) -> Result<NamesResponse>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!(
                "/lambdas/users/{}/names{}",
                address.as_ref(),
                parameters.to_query_string()
            ))
            .await?;
        Ok(response)
    }

    /// Implements [`/lambdas/users/{address}/lands`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getLands)
    pub async fn lands_for_address<T>(
        server: &Server,
        address: T,
        parameters: PageParameters,
    ) -> Result<LandsResponse>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!(
                "/lambdas/users/{}/lands{}",
                address.as_ref(),
                parameters.to_query_string()
            ))
            .await?;
        Ok(response)
    }

    /// Implements [`/lambdas/users/{address}/third-party-wearables`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getThirdPartyWearables)
    pub async fn third_party_wearables_for_address<T>(
        server: &Server,
        address: T,
        parameters: PageParameters,
    ) -> Result<ItemsResponse>
    where
        T: AsRef<str>,
    {
        let response = server
            .get(format!(
                "/lambdas/users/{}/third-party-wearables{}",
                address.as_ref(),
                parameters.to_query_string()
            ))
            .await?;
        Ok(response)
    }

    /// Implements [`/lambdas/users/{address}/third-party-wearables/{collectionId}`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getThirdPartyCollection)
    pub async fn third_party_wearables_of_collection_for_address<T, V>(
        server: &Server,
        address: T,
        collection_id: V,
        parameters: PageParameters,
    ) -> Result<ItemsResponse>
    where
        T: AsRef<str>,
        V: AsRef<str>,
    {
        let response = server
            .get(format!(
                "/lambdas/users/{}/third-party-wearables/{}{}",
                address.as_ref(),
                collection_id.as_ref(),
                parameters.to_query_string()
            ))
            .await?;
        Ok(response)
    }
    /// Implements [`/lambdas/contracts/servers`](https://decentraland.github.io/catalyst-api-specs/#operation/getServers)
    pub async fn servers(server: &Server) -> Result<Vec<Server>> {
        let servers: Vec<Server> = server.get("/lambdas/contracts/servers").await?;
        Ok(servers)
    }

    /// Implements [`/lambdas/contracts/pois`](https://peer.decentraland.org/lambdas/contracts/pois)
    pub async fn points_of_interest(server: &Server) -> Result<Vec<dcl_common::Parcel>> {
        let pois: Vec<dcl_common::Parcel> = server.get("/lambdas/contracts/pois").await?;
        Ok(pois)
    }

    /// Implements [`/lambdas/contracts/denylisted-names`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getDenylistedUserNames)
    pub async fn forbidden_names(server: &Server) -> Result<Vec<String>> {
        let forbidden_names = server.get("/lambdas/contracts/denylisted-names").await?;
        Ok(forbidden_names)
    }

    /// Implements [`/lambdas/explore/hot-scenes`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getHotScenes)
    pub async fn hot_scenes(server: &Server) -> Result<Vec<HotScene>> {
        let hot_scenes = server.get("/lambdas/explore/hot-scenes").await?;
        Ok(hot_scenes)
    }

    /// Implements [`/lambdas/explore/realms`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getRealms)
    pub async fn realms(server: &Server) -> Result<Vec<Realm>> {
        let realms = server.get("/lambdas/explore/realms").await?;
        Ok(realms)
    }

    /// Implements [`/lambdas/profiles`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getAvatarsDetailsByPost)
    pub async fn profiles(server: &Server, addresses: &[String]) -> Result<Vec<Profile>> {
        #[derive(Serialize)]
        struct Addresses {
            ids: Vec<String>,
        }

        let addresses = Addresses {
            ids: addresses.to_vec(),
        };

        let profiles: Vec<Profile> = server.post("/lambdas/profiles", &addresses).await?;
        Ok(profiles)
    }

    /// Implements [`/lambdas/profiles/{id}`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getAvatarDetails)
    pub async fn profile<T>(server: &Server, address: T) -> Result<Profile>
    where
        T: AsRef<str>,
    {
        let profile = server
            .get(format!("/lambdas/profiles/{}", &address.as_ref()))
            .await?;
        Ok(profile)
    }

    /// Implements [`/lambdas/outfits/{id}`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getOutfits)
    pub async fn outfits<T>(server: &Server, address: T) -> Result<Entity>
    where
        T: AsRef<str>,
    {
        let result = server
            .get(format!("/lambdas/outfits/{}", &address.as_ref()))
            .await?;
        Ok(result)
    }

    /// Implements [`/third-party-integrations`](https://decentraland.github.io/catalyst-api-specs/#tag/Lambdas/operation/getThirdPartyIntegrations)
    pub async fn third_party_integrations(server: &Server) -> Result<ThirdPartyIntegrations> {
        let result = server.get("/lambdas/third-party-integrations").await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use tempdir::TempDir;

    #[test]
    fn it_can_be_created() {
        LambdasClient::default();
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

        let result = tokio_test::block_on(LambdasClient::status(&server)).unwrap();

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

        let result = tokio_test::block_on(LambdasClient::collections(&server)).unwrap();

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

        tokio_test::block_on(LambdasClient::download_collection_thumbnail(
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

        tokio_test::block_on(LambdasClient::download_collection_image(
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

        let result = tokio_test::block_on(LambdasClient::erc721_entity(
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

        let result = tokio_test::block_on(LambdasClient::wearables_for_address(
            &server,
            "an-address",
            WearablesSearchParameters::default(),
        ))
        .unwrap();

        m.assert();

        let expected: ItemsResponse = serde_json::from_str(response).unwrap();
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

        let result = tokio_test::block_on(LambdasClient::emotes_for_address(
            &server,
            "an-address",
            EmotesSearchParameters::default(),
        ))
        .unwrap();

        m.assert();

        let expected: ItemsResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_names_for_address() {
        let response = include_str!("../fixtures/names_for_address.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/users/an-address/names");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::names_for_address(
            &server,
            "an-address",
            PageParameters::default(),
        ))
        .unwrap();

        m.assert();

        let expected: NamesResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_lands_for_address() {
        let response = include_str!("../fixtures/lands_for_address.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/users/an-address/lands");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::lands_for_address(
            &server,
            "an-address",
            PageParameters::default(),
        ))
        .unwrap();

        m.assert();

        let expected: LandsResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_third_party_wearables_for_address() {
        let response = include_str!("../fixtures/third_party_wearables_for_address.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/lambdas/users/an-address/third-party-wearables");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::third_party_wearables_for_address(
            &server,
            "an-address",
            PageParameters::default(),
        ))
        .unwrap();

        m.assert();

        let expected: ItemsResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_third_party_wearables_of_collection_for_address() {
        let response = include_str!("../fixtures/third_party_wearables_for_address.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/lambdas/users/an-address/third-party-wearables/a-collection");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(
            LambdasClient::third_party_wearables_of_collection_for_address(
                &server,
                "an-address",
                "a-collection",
                PageParameters::default(),
            ),
        )
        .unwrap();

        m.assert();

        let expected: ItemsResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_server_list() {
        let response = include_str!("../fixtures/servers.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/contracts/servers");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let servers = tokio_test::block_on(LambdasClient::servers(&server)).unwrap();

        m.assert();
        let expected: Vec<Server> = serde_json::from_str(response).unwrap();
        assert_eq!(servers, expected);
    }

    #[test]
    fn it_implements_pois_list() {
        let response = include_str!("../fixtures/pois.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/contracts/pois");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::points_of_interest(&server)).unwrap();

        m.assert();
        let expected: Vec<dcl_common::Parcel> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_forbidden_names_list() {
        let response = include_str!("../fixtures/forbidden_names.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/contracts/denylisted-names");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::forbidden_names(&server)).unwrap();

        m.assert();
        let expected: Vec<String> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_hot_scenes_list() {
        let response = include_str!("../fixtures/hot_scenes.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/explore/hot-scenes");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::hot_scenes(&server)).unwrap();

        m.assert();
        let expected: Vec<HotScene> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_realms() {
        let response = include_str!("../fixtures/realms.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/explore/realms");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::realms(&server)).unwrap();

        m.assert();
        let expected: Vec<Realm> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_profiles() {
        let response = include_str!("../fixtures/profiles.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/lambdas/profiles")
                .body_contains("{\"ids\":[\"id\"]}");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result =
            tokio_test::block_on(LambdasClient::profiles(&server, &vec!["id".to_string()]))
                .unwrap();

        m.assert();
        let expected: Vec<Profile> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_profile() {
        let response = include_str!("../fixtures/profile.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/profiles/id");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::profile(&server, "id")).unwrap();

        m.assert();
        let expected: Profile = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_outfits() {
        let response = include_str!("../fixtures/outfits_entity.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/outfits/id");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(LambdasClient::outfits(&server, "id")).unwrap();

        m.assert();
        let expected: Entity = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_third_party_integrations() {
        let response = include_str!("../fixtures/third_party_integrations.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/lambdas/third-party-integrations");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result =
            tokio_test::block_on(LambdasClient::third_party_integrations(&server)).unwrap();

        m.assert();
        let expected: ThirdPartyIntegrations = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }
    /*
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
