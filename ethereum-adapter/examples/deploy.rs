use walkdir::WalkDir;
use dcl_crypto::AuthChain;
use dcl2d_ecs_v1::Scene;

use std::{thread, time};

use ethereum_adapter::EthereumAdapter;
use dcl_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let mut adapter = EthereumAdapter::new();
  let mut command = std::env::current_exe().unwrap();
  
  command.pop();
  command.pop();
  adapter.start(&mut command).unwrap();
  
  // adapter.login();
  // println!("Waiting...");
  // while !adapter.is_logged_in().await {
  //   thread::sleep(time::Duration::from_millis(1000));
  //   println!("Awaiting for login...");
  // }
  
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
  let scene_files = catalyst::ContentClient::scene_files_for_parcels(
    &server, 
    &parcels
  ).await?;

  let entity_id = &scene_files.get(0).expect("Can't read entity_id from Catalyst JSON").id;
  let payload = &entity_id.0; 

  adapter.personal_sign(payload);

  while !adapter.is_signed().await {
    thread::sleep(time::Duration::from_millis(1000));
    println!("Awaiting for signature...");
  }

  let signature = adapter.signature().unwrap();
  
  // Create AuthChain
  let address_str = signature.by.address;
  let address = dcl_crypto::Address::try_from(address_str).unwrap();       
  let signature = signature.signature;

  let chain = AuthChain::simple(address, payload, signature).unwrap();
  

  println!("Deploying to Catalyst...");
  // Create list of files to deploy
  let mut files = vec![];
  for entry in WalkDir::new(&deploy_folder).into_iter().filter_map(|e| e.ok()) {
    if !entry.file_name().to_str().unwrap_or(".").starts_with(".") {
      files.push(entry.into_path())    
    }
  }

  // Figure out how to upload files.

  println!("Done with Deployment");
  Ok(())
}