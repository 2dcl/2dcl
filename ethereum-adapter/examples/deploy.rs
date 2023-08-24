use std::{
    collections::HashMap,
    thread,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Utc};
use dcl2d_ecs_v1::Scene;
use dcl_common::Result;
use dcl_crypto::{
    account::{EphemeralPayload, PersonalSignature},
    AuthChain, AuthLink, Signer,
};
use ethereum_adapter::EthereumAdapter;
use scene_deployer::*;
use walkdir::WalkDir;

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

    let mut pointers = Vec::default();
    for parcel in parcels {
        pointers.push(format!("{},{}", parcel.0, parcel.1));
    }

    let (deploy_data, entity_id) = build_entity_scene(pointers, files, &scene_files[0]);

    // Create AuthChain
    let ephemeral_identity = dcl_crypto::Account::random();
    let system_time = SystemTime::now() + Duration::from_secs(300);
    let date_time: DateTime<Utc> = system_time.into();
    let expiration_str = format!("{}", date_time.format("%Y-%m-%dT%T.000Z"));
    println!("{}", expiration_str);
    let expiration = dcl_crypto::Expiration::try_from(expiration_str).unwrap();
    let payload = EphemeralPayload::new(ephemeral_identity.address(), expiration);

    adapter.personal_sign(&payload.to_string());

    while !adapter.is_signed().await {
        thread::sleep(Duration::from_millis(1000));
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

    let payload = &entity_id.0;
    let second_signature = ephemeral_identity.sign(payload);
    chain.push(AuthLink::EcdsaPersonalSignedEntity {
        payload: payload.clone(),
        signature: second_signature,
    });

    let chain = AuthChain::from(chain);

    println!("Deploying to Catalyst...");

    let response = deploy(entity_id, deploy_data, chain, server).await.unwrap();
    println!("{:?}", response);
    // Figure out how to upload files.

    println!("Done with Deployment");
    Ok(())
}
