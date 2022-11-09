mod error;

use std::io::Write;
use std::path::Path;
use std::{fs::File, io::BufReader};
use dcl2d_ecs_v1::Scene;
use serde::Serialize;
use rmp_serde::*;
use std::fs;
use dcl_common::Result;
use crate::error::SceneCompileError;
use fs_extra::dir::CopyOptions;

pub fn compile<T,U>(source_path: T, destination_path: U)  -> Result<()>
where T: AsRef<Path>, U: AsRef<Path>
{    
    println!("Starting build...");
    
    let mut assets_source_path = source_path.as_ref().to_path_buf();
    assets_source_path.push("assets");
    let mut source_path = source_path.as_ref().to_path_buf();

    let mut assets_destination_path = destination_path.as_ref().to_path_buf();
    let mut destination_path = destination_path.as_ref().to_path_buf();

    println!("Checking build path...");

    if !source_path.exists()
    {
       return Err(Box::new(SceneCompileError::SourceNotDirectory));
    }
    else
    {
        if !source_path.is_dir()
        {
            return Err(Box::new(SceneCompileError::SourceNotDirectory));
        }
        
    }

 
    source_path.push("scene.json");


    println!("Parsing scene.json...");
    
    let file = File::open(source_path.clone())?;
    let reader = BufReader::new(file);
    let scene: Scene = serde_json::from_reader(reader)?;
    
    if scene.parcels.is_empty()
    {
        return Err(Box::new(SceneCompileError::NoParcels));
    } 

    println!("Serializing scene...");
    let mut buf: Vec<u8> = Vec::new();
    scene.serialize(&mut Serializer::new(&mut buf))?;

    //Todo check componets

    if !destination_path.exists()
    {
        fs::create_dir(&destination_path)?;
    }
    else
    {
        if !destination_path.is_dir()
        {
            return Err(Box::new(SceneCompileError::DestinationNotDirectory));
        }
    }

    println!("Writing 2dcl file...");
    destination_path.push("scene.2dcl");
    let mut file = File::create(&destination_path)?;
    
    file.write_all(&buf)?;

    let mut options = CopyOptions::new();
    options.overwrite = true;

    println!("Copying: {} -> {}", assets_source_path.display(), assets_destination_path.display());
    fs_extra::dir::copy(assets_source_path, assets_destination_path, &options)?;


    // for entity in scene.entities.iter()
    // {
    //     for component in entity.components.iter()
    //     {
    //         if let (Some(source_path), Some(destination_path)) = (source_path.parent(), destination_path.parent())
    //         {
    //             component.compile(source_path, destination_path)?;
        
    //         }
           
    //     }
    // }

  
    println!("Compilation complete.");
    Ok(())

}



#[cfg(test)]
mod tests {
    
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_no_parcel_error()
    {
        todo!("Fix This Test")
        // let json_file_path = &("./fixtures/no_parcel/scene.json");
        // let build_path = TempDir::new("build-test").unwrap();

        // let result = compile(json_file_path, build_path.path()).unwrap_err();

        // assert_eq!(result.to_string(),SceneCompileError::NoParcels.to_string());
      
    }

}
   
