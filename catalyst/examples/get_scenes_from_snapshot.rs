extern crate catalyst;

use catalyst::snapshot::EntitySnapshot;
use catalyst::*;
use dcl_common::{Parcel, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();
    let snapshot = ContentClient::snapshot(&server).await?;
    let scenes: Vec<EntitySnapshot<Parcel>> =
        ContentClient::snapshot_entities(&server, EntityType::Scene, &snapshot).await?;

    for scene in scenes {
        println!("{:?}", scene);
    }

    Ok(())
}
