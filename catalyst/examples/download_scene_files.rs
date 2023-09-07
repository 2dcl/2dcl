use std::fs;

use catalyst::*;
use dcl_common::{Parcel, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    // We will download the assets for the Genesis Plaza,
    let parcel = Parcel(0, 0);
    let scene_files = ContentClient::scene_files_for_parcels(&server, &vec![parcel]).await?;

    for scene_file in scene_files {
        let id = scene_file.id;
        fs::create_dir_all(format!("./tmp/{}", &id))?;

        for downloadable in scene_file.content {
            let filename = format!("./tmp/{}/{}", &id, downloadable.filename.to_str().unwrap());
            println!("Downloading {}", filename);

            // We're downloading this synchronously, in a production client you want to
            // store all of these and use `futures::join_all` (https://docs.rs/futures/latest/futures/future/fn.join_all.html)
            // or something of the sorts.
            ContentClient::download(&server, downloadable.cid, filename).await?;
        }
    }
    Ok(())
}
