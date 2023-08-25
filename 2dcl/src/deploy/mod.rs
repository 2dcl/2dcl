use std::{
    collections::HashMap,
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

use catalyst::EntityId;
use chrono::{DateTime, Utc};
use dcl2d_ecs_v1::Scene;
use dcl_common::Result;
use dcl_crypto::{
    account::{EphemeralPayload, PersonalSignature},
    AuthChain, AuthLink, Signer,
};
use ethereum_adapter::EthereumAdapter;
use scene_deployer::FileData;
use walkdir::WalkDir;

#[tokio::main]
pub async fn deploy<T>(deploy_folder: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let ephemeral_identity = dcl_crypto::Account::random();
    let mut chain = sign_ephemeral(&ephemeral_identity, 300)?;

    let server = catalyst::Server::production();
    let (deploy_data, entity_id) = prepare_deploy_data(deploy_folder, &server)?;

    let payload = &entity_id.0;
    let signature = ephemeral_identity.sign(payload);
    chain.push(AuthLink::EcdsaPersonalSignedEntity {
        payload: payload.clone(),
        signature,
    });

    let chain = AuthChain::from(chain);

    println!("Deploying to Catalyst...");

    let response = scene_deployer::deploy(entity_id, deploy_data, chain, server).await?;
    if response.status() == 200 {
        println!("Scene deployed");
    } else {
        println!("Scene could not be deployed");
        println!("{}", response.text().await?);
    }

    Ok(())
}

#[tokio::main]
pub async fn prepare_deploy_data<T>(
    deploy_folder: T,
    server: &catalyst::Server,
) -> Result<(Vec<FileData>, EntityId)>
where
    T: AsRef<Path>,
{
    let deploy_folder = deploy_folder.as_ref().to_path_buf();
    let mut scene_file = deploy_folder.clone();
    scene_file.push("scene.2dcl");

    let scene = std::fs::read(scene_file)?;
    let scene = Scene::from_mp(&scene)?;

    let parcels = scene.parcels;
    let scene_files = catalyst::ContentClient::scene_files_for_parcels(server, &parcels).await?;

    // Create list of files to deploy
    let mut files: HashMap<String, Vec<u8>> = HashMap::default();
    for entry in WalkDir::new(&deploy_folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_name().to_str().unwrap_or(".").starts_with('.') {
            let path = entry.into_path();
            if let Ok(bytes) = std::fs::read(&path) {
                let path_to_remove = deploy_folder.clone();
                let mut path_str = path.to_str().unwrap_or_default().to_string();
                path_str = path_str.replace(path_to_remove.to_str().unwrap_or_default(), "./2dcl");
                path_str = path_str.replace('\\', "/");
                files.insert(path_str, bytes);
            }
        }
    }

    let mut pointers = Vec::default();
    for parcel in parcels {
        pointers.push(format!("{},{}", parcel.0, parcel.1));
    }

    Ok(scene_deployer::build_entity_scene(
        pointers,
        files,
        &scene_files[0],
    ))
}

#[tokio::main]
pub async fn sign_ephemeral(
    ephemeral_identity: &dcl_crypto::Account,
    duration_in_secs: u64,
) -> Result<Vec<AuthLink>> {
    let mut adapter = EthereumAdapter::new();
    let mut command = std::env::current_exe()?;

    command.pop();
    adapter.start(&mut command)?;

    let system_time = SystemTime::now() + Duration::from_secs(duration_in_secs);
    let date_time: DateTime<Utc> = system_time.into();
    let expiration_str = format!("{}", date_time.format("%Y-%m-%dT%T.000Z"));
    let expiration = dcl_crypto::Expiration::try_from(expiration_str)?;
    let payload = EphemeralPayload::new(ephemeral_identity.address().clone(), expiration);

    adapter.personal_sign(&payload.to_string());

    while !adapter.is_signed().await {
        thread::sleep(Duration::from_millis(1000));
        println!("Awaiting for signature...");
    }

    let signature = adapter.signature().unwrap_or_default();
    let address_str = signature.by.address;
    let signature = signature.signature;

    let address = dcl_crypto::Address::try_from(address_str.clone())?;

    adapter.stop().await?;

    Ok(vec![
        AuthLink::signer(address),
        AuthLink::EcdsaPersonalEphemeral {
            payload,
            signature: PersonalSignature::try_from(signature)?,
        },
    ])
}
