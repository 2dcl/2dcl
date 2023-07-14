use walkdir::WalkDir;
use serde::Deserialize;
use std::{path::PathBuf};
use tokio::process::Command;
use dcl_common::Result;
use dcl_crypto::{Address, AuthChain};
use dcl2d_ecs_v1::Scene;

mod server;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct EthAddress {
  address: String
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum DeploySignState {
  #[default]
  NotSigning,
  WaitingForSignature,
  Signed
}

#[derive(Debug, Default)]
pub struct EthereumAdapter {
  address: Option<EthAddress>,
  deploy_signing_state: DeploySignState
}

impl EthereumAdapter {
  pub fn new() -> EthereumAdapter {
    EthereumAdapter::default()
  }

  pub fn start(&mut self, path: &mut PathBuf) -> Result<()> {
    path.push("ethereum-adapter-webserver");
    Command::new(path).spawn().expect("failed to spawn");
    Ok(())
  }

  pub fn login(&self) {
    println!("Opening browser");
    open::that("http://localhost:8000/login").unwrap();
  }

  pub async fn sign_deploy(&mut self, path: &PathBuf) -> Result<()> {
    self.deploy_signing_state = DeploySignState::WaitingForSignature;
    let mut path = path.clone();
    path.push("2dcl");
    // Create list of files to deploy
    let mut files = vec![];
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
      if !entry.file_name().to_str().unwrap_or(".").starts_with(".") {
        files.push(entry.into_path())    
      }
    }

    // Get Entity Id
    let mut scene_file = path.clone();
    scene_file.push("scene.2dcl");
    let scene = std::fs::read(scene_file)?;

    let scene = Scene::from_mp(&scene);
    println!("{:?}", scene);
    
    // Create AuthChain
    let address_str = self.address.clone().unwrap().address;
    let address = Address::try_from(address_str.to_string()).unwrap();
    let payload = "QmUsqJaHc5HQaBrojhBdjF4fr5MQc6CqhwZjqwhVRftNAo";
    let signature = "0xb962b57accc8e12083769339888f82752d13f280012b2c7b2aa2722eae103aea7a623dc88605bf7036ec8c23b0bb8f036b52f5e4e30ee913f6f2a077d5e5e3e01b";

    let chain = AuthChain::simple(address, payload, signature).unwrap();
    let owner = chain.owner().unwrap();
    

    println!("{:?}", owner);

    self.deploy_signing_state = DeploySignState::Signed;

    // assert_eq!(owner, &Address::try_from("0x4A1b9FD363dE915145008C41FA217377B2C223F2").unwrap());

    // println!("Sending files to deploy");
    // let client = reqwest::Client::new();
    // match client.post("http://localhost:8000/sign_deployment")
    //   .json(&files)
    //   .send()
    //   .await {
    //   Ok(body) => {
    //     println!("{:?}", body.json().await?);
    //   },
    //   Err(err) => { println!("ERROR {:?}", err);}
    // }

    
    Ok(())
  }

  pub fn sign_deploy_state(&self) -> DeploySignState {
    self.deploy_signing_state.clone()
  }

  pub async fn is_logged_in(&mut self) -> bool {
    println!("Requesting Address...");
    match reqwest::get("http://localhost:8000/address").await {
      Ok(body) => {
        if let Ok(address) = body.json::<EthAddress>().await {
          self.address = Some(address.clone());
        }
      },
      Err(err) => { println!("{:?}", err);}
    }
    
    self.address.is_some()
  }

  pub fn address(&self) -> Option<EthAddress> {
    self.address.clone()
  }

}
