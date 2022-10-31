use std::io::Write;
use std::error;
use std::path::Path;
use std::{fs::File, io::BufReader};
use dcl2d_ecs_v1::{Scene,Component};
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
        write!(f, "invalid first item to double")
    }
}

pub fn build<T>(json_path: T, build_path: T) -> Result<Scene,Error>
where T: AsRef<Path>,
{
    
    let mut build_path = build_path.as_ref().to_path_buf();

    if build_path.is_file()
    {
        if !build_path.exists() 
        {
            let result = fs::create_dir(build_path.parent().unwrap());
            if result.is_err()
            {
                return Err(Error::ReadFileError(result.unwrap_err()));
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

    let file = File::open(json_path.as_ref().clone());
    if file.is_ok()
    {   
        let file = file.unwrap();
        let reader = BufReader::new(file);
        let scene: serde_json::Result<Scene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();
            
            if scene.parcels.is_empty()
            {
                return Err(Error::NoParcels);
            } 

            let mut buf: Vec<u8> = Vec::new();
            scene.serialize(&mut Serializer::new(&mut buf)).unwrap();

            for entity in scene.entities.iter()
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
            }

            let mut file = File::create(build_path).unwrap();
            let file_write = file.write_all(&buf);

            if file_write.is_err()
            {
               return Err(Error::WriteFileError(file_write.unwrap_err()));
            }
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
    use std::io::Read;

    use super::*;
    use tempdir::TempDir;

    fn test_scene(path: &str)
    {
        let json_file_path = &(path.to_string() + "/scene.json");
        let build_path = TempDir::new("build-test").unwrap();

        build(Path::new(json_file_path), build_path.path()).unwrap();
    
        let mut dcl_file_path = build_path.path().to_path_buf();
        dcl_file_path.push("scene.2dcl");
        

        let mut generated_2dcl = File::open(&dcl_file_path).unwrap();
        let mut fixture_2dcl = File::open(path.to_string() + "/scene.2dcl").unwrap();
       
        let buf1 : &mut [u8] = &mut [0; 1024];
        let buf2 : &mut [u8] = &mut [0; 1024];
        generated_2dcl.read(buf1).unwrap();
        fixture_2dcl.read( buf2).unwrap();

        assert_eq!(buf1,buf2);
    }
    #[test]
    fn test_scene_name()
    {
        test_scene("./fixtures/test_scene_name");
    }

    
    #[test]
    fn test_scene_entities()
    {
        test_scene("./fixtures/test_scene_entities");
    }
    
    #[test]
    fn test_transform_component()
    {
        test_scene("./fixtures/test_transform_component");
    }

    #[test]
    fn test_sprite_renderer_component()
    {
        test_scene("./fixtures/test_sprite_renderer_component");
    }

    #[test]
    fn test_circle_collider()
    {
        test_scene("./fixtures/test_circle_collider");
    }

    #[test]
    fn test_box_collider()
    {
        test_scene("./fixtures/test_box_collider");
    }

    #[test]
    fn test_alpha_collider()
    {
        test_scene("./fixtures/test_alpha_collider");
    }
}
    
