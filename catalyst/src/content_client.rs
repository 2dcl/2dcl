use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::entity_files::SceneFile;
use crate::entity_information::EntityInformation;
use crate::snapshot::{EntitySnapshot, Snapshot};
use crate::status::ContentServerStatus;
use crate::*;
use dcl_common::{Parcel, Result};

/// Implements all the request to interact with [Catalyst Content Servers](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server).
///
#[derive(Default)]
pub struct ContentClient {}

#[derive(Serialize)]
struct ParcelPointer<'a> {
    pointers: &'a Vec<Parcel>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ContentFileStatus {
    #[serde(rename = "cid")]
    pub id: ContentId,
    pub available: bool,
}

impl ContentClient {
    /// Returns a list of entity ids related to the given ContentId hash.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getListEntityIdsByHashId)
    pub fn active_entities(server: &Server, content_id: &ContentId) -> Result<Vec<EntityId>> {
        let result = server
            .get(format!("/content/contents/{}/active-entities", content_id));
        Ok(result)
    }

    /// Returns the availability state for all the given ContentIds.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getIfFileExists)
    pub fn content_files_exists(
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
            .get(format!("/content/available-content/?{}", cids));
        Ok(result)
    }

    /// Download the file referenced by `content_id` in the path given by `filename`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getContentFile)
    pub fn download<V>(server: &Server, content_id: ContentId, filename: V) -> Result<()>
    where
        V: AsRef<Path>,
    {
        let response = server
            .raw_get(format!("/content/contents/{}", content_id));

        if let Some(parent) = filename.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let mut dest = File::create(filename)?;
        let content = response.bytes();
        dest.write_all(&content)?;

        Ok(())
    }

    /// Get information about the given `entity`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getEntityInformation)
    pub fn entity_information(server: &Server, entity: &Entity) -> Result<EntityInformation> {
        let result = server
            .get(format!("/content/audit/{}/{}", entity.kind, entity.id));
        Ok(result)
    }

    /// Returns the scene content files for all the scenes that own the given `parcels`.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getListOfEntities)
    pub fn scene_files_for_parcels(
        server: &Server,
        parcels: &Vec<Parcel>,
    ) -> Result<Vec<SceneFile>> {
        let pointers = ParcelPointer { pointers: parcels };
        let result: Vec<SceneFile> = server.post("/content/entities/active", &pointers);
        Ok(result)
    }

    /// Returns a list of entities (in the form of `EntitySnapshot`) for the given `entity_type` and `snapshot`.
    pub fn snapshot_entities<T>(
        server: &Server,
        entity_type: EntityType,
        snapshot: &Snapshot,
    ) -> Result<Vec<EntitySnapshot<T>>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let hash: &ContentId = match entity_type {
            EntityType::Scene => &snapshot.entities.scene.hash,
            EntityType::Profile => &snapshot.entities.profile.hash,
            EntityType::Wearable => &snapshot.entities.wearable.hash,
            EntityType::Emote => &snapshot.entities.emote.hash,
        };

        let response = server
            .raw_get(format!("/content/contents/{}", hash));

        let text = response.text();

        let mut result: Vec<EntitySnapshot<T>> = vec![];

        for line in text.lines() {
            if line.find('{') == Some(0) {
                let snapshot: EntitySnapshot<T> = serde_json::from_str(line)?;
                result.push(snapshot);
            }
        }

        Ok(result)
    }

    /// Returns a snapshot that includes the content ids for the entities available in the snapshot.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getActiveEntities)
    pub fn snapshot(server: &Server) -> Result<Snapshot> {
        let result = server.get("/content/snapshot");
        Ok(result)
    }

    /// Returns information about the status of the server.
    /// [See on Catalyst API Docs](https://decentraland.github.io/catalyst-api-specs/#operation/getStatus)
    pub fn status(server: &Server) -> Result<ContentServerStatus> {
        let result = server.get("/content/status");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dcl_common::Parcel;
    use httpmock::prelude::*;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn it_gets_scene_files_from_parcels() {
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
        let result: Vec<SceneFile> =
            tokio_test::block_on(ContentClient::scene_files_for_parcels(&server, &parcels))
                .unwrap();

        m.assert();

        let expected: Vec<SceneFile> = serde_json::from_str(response).unwrap();
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
    fn it_gets_snapshot() {
        let response = include_str!("../fixtures/snapshot.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path("/content/snapshot");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result: Snapshot = tokio_test::block_on(ContentClient::snapshot(&server)).unwrap();

        m.assert();

        let expected: Snapshot = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_scene_snapshot() {
        let response = include_str!("../fixtures/snapshot_entities_scene.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path(
                "/content/contents/bafybeiep3b54f6rzh5lgx647m4alfydi65smdz63y4gtpxnu2ero4trlsy",
            );
            then.status(200).body(response);
        });
        let server = Server::new(server.url(""));
        let snapshot: Snapshot =
            serde_json::from_str(include_str!("../fixtures/snapshot.json")).unwrap();

        let result: Vec<EntitySnapshot<Parcel>> = tokio_test::block_on(
            ContentClient::snapshot_entities(&server, EntityType::Scene, &snapshot),
        )
        .unwrap();

        m.assert();

        assert_eq!(result.len(), 23485);
    }

    #[test]
    fn it_gets_wearable_snapshot() {
        let response = include_str!("../fixtures/snapshot_entities_wearable.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).path(
                "/content/contents/bafybeifk2e6dsuwqz24s5bwxvhvajinr7tb7n6jzwzvafd6q4pwuy3jmua",
            );
            then.status(200).body(response);
        });
        let server = Server::new(server.url(""));
        let snapshot: Snapshot =
            serde_json::from_str(include_str!("../fixtures/snapshot.json")).unwrap();

        let result: Vec<EntitySnapshot<Urn>> = tokio_test::block_on(
            ContentClient::snapshot_entities(&server, EntityType::Wearable, &snapshot),
        )
        .unwrap();

        m.assert();

        assert_eq!(result.len(), 17325);
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
