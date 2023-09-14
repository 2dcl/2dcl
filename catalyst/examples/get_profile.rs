extern crate catalyst;

use catalyst::*;
use dcl_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();
    let address = "0x22480812a9a0669783c06d359182a583bd1d9fc2";
    let profile = LambdasClient::profile(&server, address).await?;
    println!("{:?}", profile);
    Ok(())
}
