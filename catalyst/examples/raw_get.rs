extern crate catalyst;

use catalyst::*;
use dcl_common::{ Result };

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    let scene = server.raw_get("/content/audit/scene/bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki").await?;

    let response = scene.text().await?;

    println!("{}", response);

    Ok(())
}
