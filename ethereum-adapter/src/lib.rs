use dcl_common::Result;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::process::Command;

//mod server;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct EthAddress {
    pub address: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Signature {
    pub by: EthAddress,
    pub signature: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum DeploySignState {
    #[default]
    NotSigning,
    WaitingForSignature,
    Signed,
}

#[derive(Debug, Default)]
pub struct EthereumAdapter {
    address: Option<EthAddress>,
    deploy_signing_state: DeploySignState,
    signature: Option<Signature>,
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

    pub fn login(&mut self) {
        self.address = None;
        let url = "http://localhost:8000/login";
        println!("Login at: {}", url);

        open::that(url).unwrap();
    }

    pub async fn is_logged_in(&mut self) -> bool {
        match reqwest::get("http://localhost:8000/address").await {
            Ok(body) => {
                if let Ok(address) = body.json::<EthAddress>().await {
                    self.address = Some(address.clone());
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        self.address.is_some()
    }

    pub fn address(&self) -> Option<EthAddress> {
        self.address.clone()
    }

    pub fn personal_sign(&mut self, payload: &str) {
        self.signature = None;

        let url = format!(
            "http://localhost:8000/sign?payload={}",
            payload.replace('\n', "\\n")
        );
        println!("Sign deployment: {}", url);
        open::that(url).unwrap();
    }

    pub async fn is_signed(&mut self) -> bool {
        match reqwest::get("http://localhost:8000/signature").await {
            Ok(body) => {
                if let Ok(signature) = body.json::<Signature>().await {
                    // TODO(frant): use a signature here
                    self.signature = Some(signature.clone());
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        self.signature.is_some()
    }

    pub fn signature(&self) -> Option<Signature> {
        self.signature.clone()
    }

    pub fn sign_deploy_state(&self) -> DeploySignState {
        self.deploy_signing_state.clone()
    }
}
