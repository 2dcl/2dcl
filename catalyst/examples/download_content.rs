extern crate catalyst;

use catalyst::*;

use dcl_common::{Parcel, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    // We will download the assets for the Genesis Plaza,
    let parcel = Parcel(0, 0);
    let scene_files = ContentClient::scene_files_for_parcels(&server, &vec![parcel]).await?;
    
    for scene_file in scene_files {
        for downloadable in scene_file.content {
            println!("TODO: Download {:?}", downloadable);
        }
    }
    Ok(())
}
