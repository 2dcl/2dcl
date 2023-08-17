use std::{
    collections::HashMap,
    path::PathBuf,
    str::FromStr,
    thread,
    time::{self, SystemTime, UNIX_EPOCH, Duration},
};

use catalyst::{
    entity_files::{ContentFile, DCL3dScene, SceneFile},
    ContentId, EntityId, EntityType,
};
use chrono::{DateTime, Utc};
use cid::{multihash::MultihashDigest, Cid};
use dcl2d_ecs_v1::Scene;
use dcl_common::Result;
use dcl_crypto::{
    account::{EphemeralPayload, PersonalSignature},
    AuthChain, AuthLink, Signer,
};
use ethereum_adapter::EthereumAdapter;
use walkdir::WalkDir;

struct FileData {
    cid: String,
    bytes: Vec<u8>,
    mime_str: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut adapter = EthereumAdapter::new();
    let mut command = std::env::current_exe().unwrap();

    command.pop();
    command.pop();
    adapter.start(&mut command).unwrap();

    let mut deploy_folder = std::env::current_exe().unwrap();
    deploy_folder.pop();
    deploy_folder.pop();
    deploy_folder.pop();
    deploy_folder.pop();
    deploy_folder.push("ethereum-adapter/fixtures/2dcl");

    // Create Catalyst Server Client
    let server = catalyst::Server::production();
    // Get Entity Id
    let mut scene_file = deploy_folder.clone();
    scene_file.push("scene.2dcl");

    let scene = std::fs::read(scene_file)?;
    let scene = Scene::from_mp(&scene)?;

    let parcels = scene.parcels;

    let scene_files = catalyst::ContentClient::scene_files_for_parcels(&server, &parcels).await?;

    // Create list of files to deploy
    let mut files: HashMap<String, Vec<u8>> = HashMap::default();
    for entry in WalkDir::new(&deploy_folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_name().to_str().unwrap_or(".").starts_with('.') {
            let path = entry.into_path();
            if let Ok(bytes) = std::fs::read(&path) {
                let mut a = deploy_folder.clone();
                a.pop();

                let mut path_str = path.to_str().unwrap().to_string();
                path_str = path_str.replace(a.to_str().unwrap(), ".");
                path_str = path_str.replace('\\', "/");
                files.insert(path_str, bytes);
            }
        }
    }

    let (deploy_data, entity_id) = build_entity_scene(
        scene_files[0].pointers.clone(),
        files,
        scene_files[0].metadata.clone(),
    );

    // Create AuthChain
    let ephemeral_identity = dcl_crypto::Account::random();
    let system_time = SystemTime::now() + Duration::from_secs(300);
    let date_time : DateTime<Utc> = system_time.into();
    let expiration_str = format!("{}",date_time.format("%Y-%m-%dT%T.000Z"));
    println!("{}", expiration_str);
    let expiration = dcl_crypto::Expiration::try_from(expiration_str).unwrap();
    let payload = EphemeralPayload::new(ephemeral_identity.address(), expiration);

    adapter.personal_sign(&payload.to_string());

    while !adapter.is_signed().await {
        thread::sleep(time::Duration::from_millis(1000));
        println!("Awaiting for signature...");
    }

    let signature = adapter.signature().unwrap();
    let address_str = signature.by.address;
    let signature = signature.signature;

    let address = dcl_crypto::Address::try_from(address_str.clone()).unwrap();

    let mut chain = vec![
        AuthLink::signer(address),
        AuthLink::EcdsaPersonalEphemeral {
            payload,
            signature: PersonalSignature::try_from(signature).unwrap(),
        },
    ];

    let payload = &entity_id;
    let second_signature = ephemeral_identity.sign(payload);
    chain.push(AuthLink::EcdsaPersonalSignedEntity {
        payload: payload.clone(),
        signature: second_signature,
    });

    let chain = AuthChain::from(chain);

    println!("Deploying to Catalyst...");

    let form = build_entity_form_data_for_deployment(entity_id, deploy_data, chain);

    let mut response = server
        .raw_post_form("/content/entities", form)
        .await
        .unwrap();
    println!("response: {:?}", response);
    println!("response: {:?}", response.chunk().await?);
    // Figure out how to upload files.

    println!("Done with Deployment");
    Ok(())
}

fn build_entity_form_data_for_deployment(
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

fn build_entity_scene(
    pointers: Vec<String>,
    files: HashMap<String, Vec<u8>>,
    metadata: Option<DCL3dScene>,
) -> (Vec<FileData>, String) {
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
    (files_data, entity_id)

    // prevent duplicated file names
}
