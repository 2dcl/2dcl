use std::io::Write;
use std::path::Path;
use std::{fs::File, io::BufReader};
use dcl_common::{Scene,Component};
use serde::Serialize;
use rmp_serde::*;
use std::fs;

pub fn build<T>(json_path: T, build_path: T)
where T: AsRef<Path>,
{
    
    let mut build_path = build_path.as_ref().to_path_buf();

    if let Ok(file) = File::open(json_path.as_ref().clone())
    {
        let reader = BufReader::new(file);
        let scene: serde_json::Result<Scene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();
            let mut buf: Vec<u8> = Vec::new();
            scene.serialize(&mut Serializer::new(&mut buf)).unwrap();
            if build_path.is_file()
            {
                if !build_path.exists() 
                {
                    fs::create_dir(build_path.parent().unwrap());
                }
                
                build_path.set_extension("2dcl");
            }
            else
            {
                if !build_path.exists()
                {
                    fs::create_dir(build_path.clone());
                }
                
                build_path.push("scene.2dcl");
            }
        
            for entity in scene.entities.iter()
            {
                for component in entity.components.iter()
                {
                    if let Component::AlphaCollider(alpha_collider) = component
                    {   
                        copy_asset(json_path.as_ref(), build_path.as_path(), &alpha_collider.sprite);
                    }

                    if let Component::SpriteRenderer(sprite_renderer) = component
                    {   
                        copy_asset(json_path.as_ref(), build_path.as_path(), &sprite_renderer.sprite);
                
                    }
                }
            }

            let mut file = File::create(build_path).unwrap();
            file.write_all(&buf); 
            
        }
        else 
        {
            println!("{:?}",scene);    
        }
    }
    else 
    {
        println!("Error: Json file not found");    
    }
}


fn copy_asset<P>(json_path: P, build_path: P, asset: &str)
where P: AsRef<Path>,
{
    let mut previous_path = json_path.as_ref().to_path_buf();
    previous_path.pop();
    previous_path.push(asset);

    let mut new_path = build_path.as_ref().to_path_buf();
    new_path.pop();
    new_path.push(asset);
    fs::copy(previous_path, new_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use serde::Deserialize;

    #[test]
    fn test_build_scene()
    {
        let json_file_path = "./fixtures/scene/scene.json";
        let file = File::open(json_file_path).unwrap();
        let reader = BufReader::new(file);
      
        let build_path = TempDir::new("build-test").unwrap();

        build(Path::new(json_file_path), build_path.path());
    
   
        let mut dcl_file_path = build_path.path().to_path_buf();
        dcl_file_path.push("scene.2dcl");
        
        let json_scene: serde_json::Result<Scene> = serde_json::from_reader(reader);

        let file = File::open(dcl_file_path);
        let reader = BufReader::new(file.unwrap());
        let mut de = Deserializer::new(reader);
        let dcl_scene: Result<Scene,rmp_serde::decode::Error> = Deserialize::deserialize(&mut de);
     
        assert_eq!(json_scene.unwrap().name,dcl_scene.unwrap().name);
    }

    
    #[test]
    fn test_file_completion()
    {
        
        let json_file_path = "./fixtures/scene/scene.json";
        let file = File::open(json_file_path).unwrap();
        let reader = BufReader::new(file);
      
        let build_path = TempDir::new("build-test").unwrap();


        build(Path::new(json_file_path), build_path.path());
    
        let mut initial_file_count = 0;
        for path in fs::read_dir("./fixtures/scene").unwrap()
        {
            initial_file_count+=1;
        }
       
        let mut final_file_count = 0;
        for path in fs::read_dir(&build_path).unwrap()
        {
            final_file_count+=1;
        }
       

        assert_eq!(initial_file_count,final_file_count);
    }
    
}
    