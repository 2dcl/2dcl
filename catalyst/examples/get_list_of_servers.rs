extern crate catalyst;
use catalyst::{LambdasClient, Server};
use dcl_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();
    let servers = LambdasClient::servers(&server).await?;

    for server in servers {
        println!(" - {}", server.base_url);
    }
    Ok(())
}
