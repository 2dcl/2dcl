extern crate catalyst;

use catalyst::*;
use dcl_common::Parcel;

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Scene {
    parcel_id: String,
    root_cid: String,
    scene_cid: String,
}

#[derive(Debug, Deserialize)]
struct ContentResult {
    #[serde(skip)]
    version: String,
    #[serde(rename(deserialize = "type"))]
    content_type: String,
    pointers: Vec<String>,
    timestamp: u64,
    content: Vec<ContentDescriptor>,
}

#[derive(Debug, Deserialize)]
struct ContentDescriptor {
    file: String,
    hash: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    // We will download the assets for the Genesis Plaza,
    let parcel = Parcel(-23, -1);
    let scenes = LambdaClient::scenes(&server, &parcel, &parcel).await?;

    let scene_cid = &scenes[0].scene_cid;
    // Scene Descriptor:
    // let scene_content : String = server.get(format!("/content/contents/{}", scene_cid.0)).await?;
    let scene_content = server
        .raw_get(format!("/content/contents/{}", scene_cid.0))
        .await?
        .text()
        .await?;
    println!("{}", scene_content);

    // Entities
    let entities: Vec<String> = server
        .get(format!("/content/contents/{}/active-entities", scene_cid.0))
        .await?;
    println!("{:?}", entities);
    let scene_content: ContentResult = server
        .get(format!("/content/contents/{}", entities[0]))
        .await?;

    let file_count = scene_content.content.len();
    let mut current = 0;

    for content in scene_content.content {
        let path = format!("../tmp/{}/{}", scene_cid.0, content.file);

        println!(
            "Downloading {} - ({}/{})",
            content.file, current, file_count
        );

        let path = Path::new(&path);
        if let Ok(()) = fs::create_dir_all(path.parent().unwrap()) {
            ContentClient::download(&server, content.hash, path).await?;
        }
        current += 1;
    }
    //let scene_content = ContentClient::content(&server, &scene_cid).await?;
    //   // let scene_content : ContentResult = server.get(format!("/content/contents/{}", entity)).await?;

    //   let file_count = scene_content.content.len();
    //   let mut current = 0;

    //   for content in scene_content.content {
    //     println!("Downloading {} - ({}/{}) - {}", content.file, current, file_count, entity);

    //     let path = format!("../tmp/{}/{}", entity, content.file);
    //     let path = Path::new(&path);
    //     if let Ok(()) = fs::create_dir_all(path.parent().unwrap()) {
    //       ContentClient::download(&server, content.entity, path).await?;
    //     }
    //     current += 1;
    //   }
    // }

    Ok(())
}
