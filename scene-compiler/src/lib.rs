use std::io::Write;
use std::error;
use std::path::Path;
use std::{fs::File, io::BufReader};
use dcl2d_ecs_v1::Scene;
use serde::Serialize;
use rmp_serde::*;
use std::fs::{self, read_dir};
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error
{
    NoParcels,
    BuildPathNotEmpty,
    CopyFileError(io::Error),
    WriteFileError(io::Error),
    ReadFileError(io::Error),
    DeserializeError(serde_json::Error)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self
        {
            Error::BuildPathNotEmpty => write!(f, "Error: The build path is not empty."),
            Error::NoParcels => write!(f, "Error: The property 'parcels' is empty."),
            Error::CopyFileError(e) => write!(f, "Error copying file:\n -{:?}",e),
            Error::WriteFileError(e) => write!(f, "Error writing file:\n -{:?}",e),
            Error::ReadFileError(e) => write!(f, "Error reading file:\n -{:?}",e),
            Error::DeserializeError(e) => write!(f, "Error deserializing json file:\n -{:?}",e),

        }
        
    }
}  


pub fn build<T>(json_path: T, build_path: T) -> Result<Scene,Error>
where T: AsRef<Path>,
{    
    println!("Starting build...");
    
    let mut build_path = build_path.as_ref().to_path_buf();

    println!("Checking build path...");

    
    if build_path.is_file()
    {
        if !build_path.exists() 
        {
            let result = fs::create_dir(build_path.parent().unwrap());
            if result.is_err()
            {
                return Err(Error::WriteFileError(result.unwrap_err()));
            }
        }
        else
        {
            return Err(Error::BuildPathNotEmpty);
        }
        
        build_path.set_extension("2dcl");
    }
    else
    {
        if !build_path.exists()
        {
            let result = fs::create_dir(&build_path);
            if result.is_err()
            {
                return Err(Error::WriteFileError(result.unwrap_err()));
            }
        }
        else
        {
           let dir_content = read_dir(&build_path);
           
            if dir_content.is_err()
            {
                return Err(Error::ReadFileError(dir_content.unwrap_err()));
            }
            else
            {
                let mut dir_content = dir_content.unwrap();
                if dir_content.next().is_some()
                {
                    return Err(Error::BuildPathNotEmpty);
                }
            }
            
        }
        
        build_path.push("scene.2dcl");
    }

    println!("Opening json file...");


    let mut json_path = json_path.as_ref().to_path_buf();
    if !json_path.is_file()
    {
        json_path.push("scene.json");
    }
    
    let file = File::open(json_path.clone());
    if file.is_ok()
    {   
        let file = file.unwrap();
        let reader = BufReader::new(file);
        let scene: serde_json::Result<Scene> = serde_json::from_reader(reader);
        println!("Reading json file...");
        if scene.is_ok()
        {
            let scene = scene.unwrap();
            
            if scene.parcels.is_empty()
            {
                return Err(Error::NoParcels);
            } 

            let mut buf: Vec<u8> = Vec::new();
            scene.serialize(&mut Serializer::new(&mut buf)).unwrap();

            println!("Serializing scene...");

            //Copy all files?   
            
            
            /*for entity in scene.entities.iter()
            {
                for component in entity.components.iter()
                {
                    
                   if let Component::AlphaCollider(alpha_collider) = component
                    {   
                        let result = copy_asset(json_path.as_ref(), build_path.as_path(), &alpha_collider.sprite);
                        if result.is_err()
                        {
                            return Err(result.unwrap_err());
                        }
                    }

                    if let Component::SpriteRenderer(sprite_renderer) = component
                    {   
                        let result = copy_asset(json_path.as_ref(), build_path.as_path(), &sprite_renderer.sprite);
                        if result.is_err()
                        {
                            return Err(result.unwrap_err());
                        }
                    }  
                }
            }*/

            let mut file = File::create(build_path).unwrap();
            let file_write = file.write_all(&buf);

            println!("Writing 2dcl file...");
            if file_write.is_err()
            {
               return Err(Error::WriteFileError(file_write.unwrap_err()));
            }
            println!("Compilation complete.");
            return Ok(scene);
        }
        else 
        {  
            return  Err(Error::DeserializeError(scene.unwrap_err()));
        }
    }

    Err(Error::ReadFileError(file.unwrap_err()))

}


fn copy_asset<P>(json_path: P, build_path: P, asset: &str) -> Result<u64,Error>
where P: AsRef<Path>,
{
    println!("Moving {}, to {}",asset,build_path.as_ref().to_str().unwrap());
    let mut previous_path = json_path.as_ref().to_path_buf();
    previous_path.pop();
    previous_path.push(asset);

    let mut new_path = build_path.as_ref().to_path_buf();
    new_path.pop();
    new_path.push(asset);
    let result = fs::copy(previous_path, new_path);
    if result.is_err()
    {
        return Err(Error::CopyFileError(result.unwrap_err()));
    }

    Ok(result.unwrap())
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_no_parcel_error()
    {

        let json_file_path = &("./fixtures/no_parcel/scene.json");
        let build_path = TempDir::new("build-test").unwrap();

        let result = build(Path::new(json_file_path), build_path.path()).unwrap_err();

        assert_eq!(result.to_string(),Error::NoParcels.to_string());
      
    }

    #[test]
    fn test_build_path_not_empty_error()
    {

        let json_file_path = "./fixtures/build_path_not_empty/scene.json";
        let build_path = "./fixtures/build_path_not_empty";

        let result = build(json_file_path, build_path).unwrap_err();

        assert_eq!(result.to_string(),Error::BuildPathNotEmpty.to_string());
    }

}
    
