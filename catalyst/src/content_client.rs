use dcl_crypto::AuthChain;
use reqwest::multipart::Form;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::deployment::Deployment;
use crate::entity::{Entity, EntityId, EntityType};
use crate::entity_information::EntityInformation;
use crate::snapshot::Snapshot;
use crate::status::ContentServerStatus;
use crate::*;
use dcl_common::{Parcel, Result};

/// Implements all the request to interact with [Catalyst Content Servers](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server).
///
#[derive(Default)]
pub struct ContentClient {}

#[derive(Serialize)]
pub struct Pointers {
    pub pointers: Vec<String>,
}

impl Pointers {
    pub fn from_parcels(parcels: &Vec<Parcel>) -> Self {
        let mut pointers = Vec::default();
        for parcel in parcels {
            pointers.push(format!("{},{}", parcel.0, parcel.1));
        }
        Pointers { pointers }
    }
}

#[derive(Serialize)]
pub struct Ids {
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub challenge_text: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeployResponse {
    pub creation_timestamp: u128,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesByUrnResponse {
    pub total: u64,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangesResponse {
    pub deltas: Vec<Deployment>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FailedDeployment {
    pub entity_type: EntityType,
    pub entity_id: String,
    pub failure_timestamp: u128,
    pub reason: String,
    pub auth_chain: AuthChain,
    pub error_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_hash: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ContentFileStatus {
    #[serde(rename = "cid")]
    pub id: ContentId,
    pub available: bool,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ChangesSearchParameters {
    from: Option<u64>,
    to: Option<u64>,
    last_id: Option<EntityId>,
    limit: Option<u64>,
    entity_type: Option<EntityType>,
    sorting_field: Option<SortingField>,
    sorting_order: Option<SortingOrder>,
}

impl ChangesSearchParameters {
    pub fn to_query_string(&self) -> String {
        let mut query_string = "?".to_string();
        if let Some(from) = &self.from {
            query_string += &format!("from={}&", from);
        }
        if let Some(to) = &self.to {
            query_string += &format!("to={}&", to);
        }
        if let Some(last_id) = &self.last_id {
            query_string += &format!("lastId={}&", last_id.0);
        }
        if let Some(limit) = &self.limit {
            query_string += &format!("limit={}&", limit);
        }
        if let Some(entity_type) = &self.entity_type {
            query_string += &format!("entityType={}&", entity_type);
        }
        if let Some(sorting_field) = &self.sorting_field {
            query_string += &format!("sortingField={}&", sorting_field);
        }
        if let Some(sorting_order) = &self.sorting_order {
            query_string += &format!("sortingField={}", sorting_order);
        }
        query_string
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum SortingOrder {
    #[default]
    Ascending,
    Descending,
}

impl fmt::Display for SortingOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialization = match self {
            SortingOrder::Ascending => "ASC",
            SortingOrder::Descending => "DESC",
        };
        write!(f, "{}", serialization)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum SortingField {
    #[default]
    LocalTimestamp,
    EntityTimestamp,
}

impl fmt::Display for SortingField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialization = match self {
            SortingField::LocalTimestamp => "local_timestamp",
            SortingField::EntityTimestamp => "entity_timestamp",
        };
        write!(f, "{}", serialization)
    }
}

impl ContentClient {
    /// Returns a list of entity ids related to the given ContentId hash.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getListEntityIdsByHashId)
    pub async fn active_entities(server: &Server, content_id: &ContentId) -> Result<Vec<EntityId>> {
        let result = server
            .get(format!("/content/contents/{}/active-entities", content_id))
            .await?;
        Ok(result)
    }

    ///Used by the Server to figure out their identity on the DAO by themselves, so they will generate a random challenge text, and then query each server for it. If the text matches, then they have found themselves.
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/getContentFile)
    pub async fn challenge(server: &Server) -> Result<Challenge> {
        let result = server.get("/content/challenge").await?;
        Ok(result)
    }

    /// Returns true if the content exists and false if it doesnt.
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/headContentFile)    pub async fn challenge(server: &Server) -> Result<Challenge> {
    pub async fn content_file_exists(server: &Server, content: &ContentId) -> Result<bool> {
        let result = server
            .raw_head(format!("/content/contents/{}", content.hash()))
            .await?;

        Ok(result.status() == StatusCode::OK)
    }

    /// Returns the entity ids whose deployments are associated with the specified content hash.
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/getEntityIdsByHashId)
    pub async fn entity_ids_by_hash(server: &Server, hash: &HashId) -> Result<Vec<ContentId>> {
        let result: Vec<ContentId> = server
            .get(format!("/content/contents/{}/entities", hash))
            .await?;
        Ok(result)
    }

    /// Deploys an entity in the content server.
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/postEntity)
    pub async fn deploy_entity(server: &Server, form: Form) -> Result<DeployResponse> {
        let result = server.post_form("/content/entities", form).await?;
        Ok(result)
    }

    /// Returns the list of active entities which have at least one pointer that matches the prefix given
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/getEntitiesByPointerPrefix)
    pub async fn entities_by_urn<T>(server: &Server, urn: T) -> Result<EntitiesByUrnResponse>
    where
        T: AsRef<str>,
    {
        let result = server
            .get(format!(
                "/content/entities/active/collections/{}",
                urn.as_ref()
            ))
            .await?;
        Ok(result)
    }

    /// Retrieves a list of the failed deployments
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/getFailedDeployments)
    pub async fn failed_deployments(server: &Server) -> Result<Vec<FailedDeployment>> {
        let result: Vec<FailedDeployment> = server
            .get("/content/failed-deployments".to_string())
            .await?;
        Ok(result)
    }

    /// It returns a list of changes with the before field (the entity that was overridden with this deployment) and after (the entity that overrides the current one if present).
    ///[See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/getPointerChanges)
    pub async fn changes(
        server: &Server,
        parameters: ChangesSearchParameters,
    ) -> Result<ChangesResponse> {
        println!("/content/pointer-changes/{}", parameters.to_query_string());
        let result = server
            .get(format!(
                "/content/pointer-changes/{}",
                parameters.to_query_string()
            ))
            .await?;
        Ok(result)
    }

    /// Returns the availability state for all the given ContentIds.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getIfFileExists)
    pub async fn content_files_exists(
        server: &Server,
        content: &Vec<ContentId>,
    ) -> Result<Vec<ContentFileStatus>> {
        let mut cids = String::new();

        for cid in content {
            if cid != &content[0] {
                cids.push('&');
            }
            cids.push_str("cid=");
            cids.push_str(cid.hash());
        }

        let result = server
            .get(format!("/content/available-content/?{}", cids))
            .await?;
        Ok(result)
    }

    /// Download the file referenced by `content_id` in the path given by `filename`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getContentFile)
    pub async fn download<V>(server: &Server, content_id: ContentId, filename: V) -> Result<()>
    where
        V: AsRef<Path>,
    {
        let response = server
            .raw_get(format!("/content/contents/{}", content_id))
            .await?;

        if let Some(parent) = filename.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let mut dest = File::create(filename)?;
        let content = response.bytes().await?;
        dest.write_all(&content)?;

        Ok(())
    }

    /// Get information about the given `entity`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getEntityInformation)
    pub async fn entity_information(server: &Server, entity: &Entity) -> Result<EntityInformation> {
        let result = server
            .get(format!("/content/audit/{}/{}", entity.kind, entity.id))
            .await?;
        Ok(result)
    }

    /// Returns the entities for all the given `pointers`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getListOfEntities)
    pub async fn entities_for_pointers(
        server: &Server,
        pointers: &Pointers,
    ) -> Result<Vec<Entity>> {
        let result: Vec<Entity> = server.post("/content/entities/active", pointers).await?;
        Ok(result)
    }

    /// Returns the entities for all the given `ids`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getListOfEntities)
    pub async fn entities_for_ids(server: &Server, ids: &Ids) -> Result<Vec<Entity>> {
        let result: Vec<Entity> = server.post("/content/entities/active", ids).await?;
        Ok(result)
    }

    /// Returns the scene entities for all the scenes that own the given `parcels`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getListOfEntities)
    pub async fn scene_entities_for_parcels(
        server: &Server,
        parcels: &Vec<Parcel>,
    ) -> Result<Vec<Entity>> {
        let pointers = Pointers::from_parcels(parcels);
        let result: Vec<Entity> = Self::entities_for_pointers(server, &pointers).await?;
        Ok(result)
    }

    /// Returns all active deployments stored in the database in multiple snapshots for different time ranges.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server/operation/getSnapshots)
    pub async fn snapshots(server: &Server) -> Result<Vec<Snapshot>> {
        let result = server.get("/content/snapshots").await?;
        Ok(result)
    }

    /// Returns information about the status of the server.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getStatus)
    pub async fn status(server: &Server) -> Result<ContentServerStatus> {
        let result = server.get("/content/status").await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use dcl_common::Parcel;
    use httpmock::prelude::*;
    use httpmock::Method::HEAD;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn it_gets_entities_from_ids() {
        let response = include_str!("../fixtures/scenes_from_parcels.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/content/entities/active")
                .body_contains("{\"ids\":[\"id\"]}");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let ids = Ids {
            ids: vec!["id".to_string()],
        };
        let result: Vec<Entity> =
            tokio_test::block_on(ContentClient::entities_for_ids(&server, &ids)).unwrap();

        m.assert();

        let expected: Vec<Entity> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_entities_from_pointers() {
        let response = include_str!("../fixtures/scenes_from_parcels.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/content/entities/active")
                .body_contains("{\"pointers\":[\"a-pointer\"]}");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let pointers = Pointers {
            pointers: vec!["a-pointer".to_string()],
        };
        let result: Vec<Entity> =
            tokio_test::block_on(ContentClient::entities_for_pointers(&server, &pointers)).unwrap();

        m.assert();

        let expected: Vec<Entity> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_entities_from_parcels() {
        let response = include_str!("../fixtures/scenes_from_parcels.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/content/entities/active")
                .body_contains("{\"pointers\":[\"0,0\"]}");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let parcels = vec![Parcel(0, 0)];
        let result: Vec<Entity> =
            tokio_test::block_on(ContentClient::scene_entities_for_parcels(&server, &parcels))
                .unwrap();

        m.assert();

        let expected: Vec<Entity> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_challenges() {
        let response = include_str!("../fixtures/challenge.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/challenge");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(ContentClient::challenge(&server)).unwrap();

        m.assert();

        let expected: Challenge = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_content_file_exists() {
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(HEAD).path("/content/contents/a-cid");
            then.status(200);
        });

        let server = Server::new(server.url(""));
        let content_id = ContentId::new("a-cid".to_string());
        let result =
            tokio_test::block_on(ContentClient::content_file_exists(&server, &content_id)).unwrap();

        m.assert();
        assert!(result);

        let content_id = ContentId::new("invalid_cid".to_string());
        let result =
            tokio_test::block_on(ContentClient::content_file_exists(&server, &content_id)).unwrap();

        assert!(!result);
    }

    #[test]
    fn it_gets_entitiy_ids_by_hash() {
        let response = include_str!("../fixtures/entities_by_hash.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/contents/a-cid/entities");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(ContentClient::entity_ids_by_hash(
            &server,
            &"a-cid".to_string(),
        ))
        .unwrap();

        m.assert();

        let expected: Vec<ContentId> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_deploys() {
        let response = include_str!("../fixtures/deploy_timestamp.json");

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST).path("/content/entities");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result =
            tokio_test::block_on(ContentClient::deploy_entity(&server, Form::default())).unwrap();

        m.assert();

        let expected: DeployResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implement_changes() {
        let response = include_str!("../fixtures/changes.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/pointer-changes/");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(ContentClient::changes(
            &server,
            ChangesSearchParameters::default(),
        ))
        .unwrap();

        m.assert();

        let expected: ChangesResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_entities_by_urn() {
        let response = include_str!("../fixtures/entities_by_urn.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/entities/active/collections/a-urn");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result =
            tokio_test::block_on(ContentClient::entities_by_urn(&server, "a-urn")).unwrap();

        m.assert();

        let expected: EntitiesByUrnResponse = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_failed_deployments() {
        let response = include_str!("../fixtures/failed_deployments.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/failed-deployments");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let result = tokio_test::block_on(ContentClient::failed_deployments(&server)).unwrap();

        m.assert();

        let expected: Vec<FailedDeployment> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_implements_content_files_exist() {
        let response = include_str!("../fixtures/available_content.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/available-content/")
                .query_param("cid", "a-cid")
                .query_param("cid", "another-cid");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let cids = vec![ContentId::new("a-cid"), ContentId::new("another-cid")];

        let result: Vec<ContentFileStatus> =
            tokio_test::block_on(ContentClient::content_files_exists(&server, &cids)).unwrap();

        m.assert();

        let expected = vec![
            ContentFileStatus {
                id: ContentId::new("a-cid"),
                available: true,
            },
            ContentFileStatus {
                id: ContentId::new("another-cid"),
                available: false,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_entity_information() {
        let response = include_str!("../fixtures/audit_scene_result.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/audit/scene/id");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let entity = Entity::scene("id");
        let result: EntityInformation =
            tokio_test::block_on(ContentClient::entity_information(&server, &entity)).unwrap();

        m.assert();

        let expected: EntityInformation = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_active_entities() {
        let response = "[\"entity-id\"]";
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/contents/an-id/active-entities");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let content_id = ContentId::new("an-id");
        let result =
            tokio_test::block_on(ContentClient::active_entities(&server, &content_id)).unwrap();

        m.assert();

        assert_eq!(result, vec!(EntityId::new("entity-id")));
    }

    #[test]
    fn it_gets_status() {
        let response = include_str!("../fixtures/content_server_status.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/status");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: ContentServerStatus =
            tokio_test::block_on(ContentClient::status(&server)).unwrap();

        m.assert();

        let expected: ContentServerStatus = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_snapshots() {
        let response = include_str!("../fixtures/snapshots.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/snapshots");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: Vec<Snapshot> =
            tokio_test::block_on(ContentClient::snapshots(&server)).unwrap();

        m.assert();

        let expected: Vec<Snapshot> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
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

        tokio_test::block_on(ContentClient::download(
            &server,
            ContentId::new("a-hash"),
            filename.clone(),
        ))
        .unwrap();

        m.assert();

        assert_eq!(fs::read_to_string(filename).unwrap(), "File Content");
    }
}
