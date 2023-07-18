use dcl_common::Result;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::process::Command;

mod server;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct EthAddress {
    pub address: String,
}

#[derive(Debug, Default)]
pub struct EthereumAdapter {
    address: Option<EthAddress>,
}

impl EthereumAdapter {
    pub fn start(&mut self, path: &mut PathBuf) -> Result<()> {
        path.push("ethereum-adapter-webserver");
        Command::new(path).spawn().expect("failed to spawn");
        Ok(())
    }

    pub fn login(&self) {
        println!("Opening browser");
        open::that("http://localhost:8000/login").unwrap();
    }

    pub async fn is_logged_in(&mut self) -> bool {
        println!("Requesting Address...");
        match reqwest::get("http://localhost:8000/address").await {
            Ok(body) => {
                if let Ok(address) = body.json::<EthAddress>().await {
                    self.address = Some(address);
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
}
