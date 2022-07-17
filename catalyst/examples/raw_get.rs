extern crate catalyst;

use catalyst::*;
use dcl_common::{ Result };

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    let scene = server.raw_get("/content/contents/bafybeiep3b54f6rzh5lgx647m4alfydi65smdz63y4gtpxnu2ero4trlsy").await?;

    let response = scene.text().await?;

    println!("{}", response);

    Ok(())
}
