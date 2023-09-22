extern crate catalyst;

use catalyst::*;
use dcl_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    let scene = server.raw_get("/content/available-content/?cid=QmWFLwHGfvhB9a1epaRpS38HEwbHvhpaYzHEsNhDRgon7P&cid=MfWFLwHGfvhB9a1epaRpJ38HEwbHvhpaYzHEsNhDRgon8H").await?;

    let response = scene.text().await?;

    println!("{}", response);

    Ok(())
}
