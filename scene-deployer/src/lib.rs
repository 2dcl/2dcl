mod error;
use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use catalyst::{
    entity_files::{ContentFile, DCL3dScene, SceneFile},
    ContentId, EntityId, EntityType,
};
use cid::{multihash::MultihashDigest, Cid};
use dcl_crypto::{AuthChain, AuthLink};
use reqwest::Response;

pub struct FileData {
    cid: String,
    bytes: Vec<u8>,
    mime_str: String,
}

pub async fn deploy(
    entity_id: EntityId,
    deploy_data: Vec<FileData>,
    auth_chain: AuthChain,
    server: catalyst::Server,
) -> Result<Response, Box<dyn Error>> {
    let form =
        build_entity_form_data_for_deployment(entity_id.to_string(), deploy_data, auth_chain);
    server.raw_post_form("/content/entities", form).await
}

pub fn build_entity_form_data_for_deployment(
    entity_id: String,
    deploy_data: Vec<FileData>,
    auth_chain: AuthChain,
) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();
    form = form.part(
        "entityId",
        reqwest::multipart::Part::text(entity_id.clone()),
    );

    for (index, auth_link) in (0..).zip(auth_chain.iter()) {
        form = form.part(
            format!("authChain[{}][type]", index),
            reqwest::multipart::Part::text((*auth_link.kind()).to_string()),
        );
        match auth_link {
            AuthLink::Signer { payload, signature } => {
                form = form.part(
                    format!("authChain[{}][payload]", index),
                    reqwest::multipart::Part::text(payload.to_string()),
                );
                form = form.part(
                    format!("authChain[{}][signature]", index),
                    reqwest::multipart::Part::text(signature.clone()),
                );
            }
            AuthLink::EcdsaPersonalEphemeral { payload, signature } => {
                form = form.part(
                    format!("authChain[{}][payload]", index),
                    reqwest::multipart::Part::text(payload.to_string()),
                );
                form = form.part(
                    format!("authChain[{}][signature]", index),
                    reqwest::multipart::Part::text(signature.to_string()),
                );
            }
            AuthLink::EcdsaPersonalSignedEntity { payload, signature } => {
                form = form.part(
                    format!("authChain[{}][payload]", index),
                    reqwest::multipart::Part::text(payload.to_string()),
                );
                form = form.part(
                    format!("authChain[{}][signature]", index),
                    reqwest::multipart::Part::text(signature.to_string()),
                );
            }
            AuthLink::EcdsaEip1654Ephemeral { payload, signature } => {
                form = form.part(
                    format!("authChain[{}][payload]", index),
                    reqwest::multipart::Part::text(payload.to_string()),
                );
                form = form.part(
                    format!("authChain[{}][signature]", index),
                    reqwest::multipart::Part::text(signature.to_string()),
                );
            }
            AuthLink::EcdsaEip1654SignedEntity { payload, signature } => {
                form = form.part(
                    format!("authChain[{}][payload]", index),
                    reqwest::multipart::Part::text(payload.to_string()),
                );
                form = form.part(
                    format!("authChain[{}][signature]", index),
                    reqwest::multipart::Part::text(signature.to_string()),
                );
            }
        }
    }

    for file in deploy_data {
        let part = reqwest::multipart::Part::bytes(file.bytes)
            .file_name(file.cid.clone())
            .mime_str(&file.mime_str)
            .unwrap();

        form = form.part(file.cid.clone(), part);
    }

    form
}

fn get_cid(content: &[u8]) -> String {
    let codec: u64 = 0x55;
    let h = cid::multihash::Code::Sha2_256.digest(content);
    Cid::new_v1(codec, h).to_string()
}

pub fn build_entity_scene(
    pointers: Vec<String>,
    files: HashMap<String, Vec<u8>>,
    metadata: Option<DCL3dScene>,
) -> (Vec<FileData>, EntityId) {
    let mut content = Vec::default();
    let mut files_data = Vec::default();
    for (path, bytes) in files {
        let cid = get_cid(&bytes);
        content.push(ContentFile {
            filename: PathBuf::from_str(&path).unwrap(),
            cid: ContentId::new(cid.clone()),
        });

        let mime_str = match path.ends_with(".png") {
            true => "image/png".to_string(),
            false => "application/octet-stream".to_string(),
        };
        files_data.push(FileData {
            cid,
            bytes,
            mime_str,
        });
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut entity = SceneFile {
        id: None,
        version: "v3".to_string(),
        kind: EntityType::Scene,
        pointers,
        timestamp,
        content,
        metadata,
    };

    let entity_file = serde_json::to_string(&entity).unwrap();
    let entity_id = get_cid(entity_file.as_bytes());
    entity.id = Some(EntityId(entity_id.clone()));

    files_data.push(FileData {
        cid: entity_id.clone(),
        bytes: entity_file.as_bytes().to_vec(),
        mime_str: "application/octet-stream".to_string(),
    });
    (files_data, EntityId(entity_id))

    // prevent duplicated file names
}
