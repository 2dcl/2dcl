

use std::fs;

use catalyst::*;
use dcl_common::{Parcel, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    // We will download the assets for the Genesis Plaza,
    let address = dcl_crypto::Address::try_from(
      "0x270722b5222968603E4650C3b70A7DfB971Ed5B6"
    ).unwrap();
    
    let profile = LambdaClient::profile(&server, address).await?;

    println!("{}", profile);
    // for scene_file in scene_files {
    //     fs::create_dir_all(format!("./tmp/{}", scene_file.id))?;

    //     for downloadable in scene_file.content {
    //         let filename = format!(
    //             "./tmp/{}/{}",
    //             scene_file.id,
    //             downloadable.filename.to_str().unwrap()
    //         );
    //         println!("Downloading {}", filename);

    //         // We're downloading this synchronously, in a production client you want to
    //         // store all of these and use `futures::join_all` (https://docs.rs/futures/latest/futures/future/fn.join_all.html)
    //         // or something of the sorts.
    //         ContentClient::download(&server, downloadable.cid, filename).await?;
    //     }
    // }
    Ok(())
}
