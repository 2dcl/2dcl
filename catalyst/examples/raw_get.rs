extern crate catalyst;

use catalyst::*;
use dcl_common::{ Result };

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    let scene = server.raw_get("/content/snapshot").await?;

    let response = scene.text().await?;

    println!("{}", response);

    Ok(())
}
