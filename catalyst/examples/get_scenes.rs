extern crate catalyst;

use catalyst::*;
use dcl_common::{Parcel, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();
    let parcel = Parcel(0, 0);
    let scenes = ContentClient::scene_files_for_parcels(&server, &vec![parcel]).await?;

    for scene in scenes {
        println!(" - {:?}", scene);
    }
    Ok(())
}
