use std::path::{PathBuf, Path};

use bevy::prelude::*;
use catalyst::*;
use dcl_common::Parcel;

use crate::renderer::{scenes_io::read_3dcl_scene, scene_maker::add_road_at_parcel};

use super::scene_maker::{RoadsData, read_roads_data};


pub fn update_roads( 
  mut commands: Commands)
{
  match read_roads_data() {
    Ok(mut roads_data) => 
    {
      commands.insert_resource(roads_data.clone());
      update_roads_async(&mut roads_data);
    }

    Err(e) => println!("error:{}", e),
  }

}

#[tokio::main]
pub async fn update_roads_async(
   roads_data: &mut RoadsData,
) {

  println!("updating roads async");
  for x in -9..152 {
   for y in -152..152 {
    let parcels = vec![Parcel(x,y)];
    println!("checking parcel {:?}",parcels);
    let server = Server::production();

    let scene_files = ContentClient::scene_files_for_parcels(&server, &parcels).await.unwrap();

    for scene_file in scene_files {
      let path_str = "./assets/scenes/".to_string() + &scene_file.id.to_string();
      let scene_path = Path::new(&path_str);
      
      if !scene_path.exists() {
        std::fs::create_dir_all(format!("./assets/scenes/{}", scene_file.id));
    
     
        for downloadable in scene_file.clone().content {
            if downloadable
                .filename
                .to_str()
                .unwrap()
                .ends_with("scene.json")
            {

              
              let filename = format!(
                "./assets/scenes/{}/{}",
                scene_file.id,
                downloadable.filename.to_str().unwrap()
                );

              println!("Downloading {}", filename);
              ContentClient::download(&server, downloadable.cid, &filename).await;
              
              if let Ok(scene_3d) = read_3dcl_scene(filename) {
                
                if scene_3d.display.title.to_lowercase().contains("road")
                {
                  for parcel in scene_3d.scene.parcels
                  {
                    println!("{:?} is road", parcel);
                    add_road_at_parcel(&parcel,roads_data);
                  }
              
                }
            }
        }
      }
    }
  }
  }
}
}