use catalyst::entity_files::ContentFile;
use catalyst::{ContentClient, Server};
use dcl_common::Parcel;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::renderer::scenes_io::read_scene_file;

pub fn where_command() -> dcl_common::Result<()> {

  let mut output_file = File::create("where.txt")?;
  if let Ok(Some(result)) = print_2dcl_scene(Parcel(0,0))
  { 
    println!("{}",result);
    output_file.write(result.as_bytes())?;
  }
  
  for distance_from_center in 1..152{

    if let Ok(Some(result)) = print_2dcl_scene(Parcel(0,distance_from_center))
    { 
      println!("{}",result);
      output_file.write(result.as_bytes())?;
    }
    if let Ok(Some(result)) = print_2dcl_scene(Parcel(distance_from_center,0))
    { 
      println!("{}",result);
      output_file.write(result.as_bytes())?;
    }

    for x in 1..distance_from_center
    {
      if let Ok(Some(result)) = print_2dcl_scene(Parcel(x,distance_from_center))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }
      if let Ok(Some(result)) = print_2dcl_scene(Parcel(-x,distance_from_center))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }

      if let Ok(Some(result)) = print_2dcl_scene(Parcel(x,-distance_from_center))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }
      if let Ok(Some(result)) = print_2dcl_scene(Parcel(-x,-distance_from_center))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }
    }
    for y in 1..distance_from_center-1{

      if let Ok(Some(result)) = print_2dcl_scene(Parcel(distance_from_center,y))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }

      if let Ok(Some(result)) = print_2dcl_scene(Parcel(distance_from_center,-y))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }
      if let Ok(Some(result)) = print_2dcl_scene(Parcel(-distance_from_center,y))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }

      if let Ok(Some(result)) = print_2dcl_scene(Parcel(-distance_from_center,-y))
      { 
        println!("{}",result);
        output_file.write(result.as_bytes())?;
      }
    }
    }

    Ok(())
}

#[tokio::main]
async fn print_2dcl_scene(parcel: Parcel) -> dcl_common::Result<Option<String>>{

  println!("checking parcel ({}, {})",parcel.0,parcel.1);
  let server = Server::production();
  let scene_files = ContentClient::scene_files_for_parcels(&server, &vec![parcel.clone()]).await?;

  let mut output_string = None;
  for scene_file in scene_files {
      let path_str = "./assets/scenes/".to_string() + &scene_file.id.to_string();
      let scene_path = Path::new(&path_str);
      let mut downloadable_2dcl: Option<ContentFile> = None;

      for downloadable in scene_file.clone().content {
          if downloadable
              .filename
              .to_str()
              .unwrap()
              .ends_with("scene.2dcl")
          {
              downloadable_2dcl = Some(downloadable);
          }
      }

      if !scene_path.exists() {
          fs::create_dir_all(format!("./assets/scenes/{}", scene_file.id))?;
      }

      if let Some(downloadable_2dcl) = downloadable_2dcl
      {
          let filename = format!(
            "./assets/scenes/{}/{}-temp",
            scene_file.id,
            downloadable_2dcl.filename.to_str().unwrap()
          );

          ContentClient::download(&server, downloadable_2dcl.cid, &filename).await?;
          if let Some(scene_2cl) = read_scene_file(&filename) {
            let scene_name = scene_2cl.name;
            output_string = Some(format!("({}, {}) -> {}",&parcel.0,&parcel.1,scene_name));
          }

          match std::fs::remove_file(filename) {
              Ok(_) => {}
              Err(e) => println!("{}", e),
          };
      }
  }
  
  Ok(output_string)
}