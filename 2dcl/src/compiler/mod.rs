use std::{path::PathBuf, fs::File, io::BufReader};
use crate::renderer::scene_loader::{self, EntityComponent};
use serde::Serialize;
use rmp_serde::*;
use std::fs;
use std::io::prelude::*;

pub fn run(json_path: PathBuf, mut build_path: PathBuf) {

   
    if let Ok(file) = File::open(json_path.clone())
    {
        let reader = BufReader::new(file);
        let scene: serde_json::Result<scene_loader::Scene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let mut scene = scene.unwrap();
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
                    if let EntityComponent::AlphaCollider(alpha_collider) = component
                    {   
                        let mut previous_path = json_path.clone();
                        previous_path.pop();
                        previous_path.push(alpha_collider.sprite.clone());

                        let mut new_path = build_path.clone();
                        new_path.pop();
                        new_path.push(alpha_collider.sprite.clone());
                        fs::copy(previous_path, new_path);
                    }

                    if let EntityComponent::AsepriteAnimation(animation) = component
                    {   
                        let mut previous_path = json_path.clone();
                        previous_path.pop();
                        previous_path.push(animation.json_path.clone());

                        let mut new_path = build_path.clone();
                        new_path.pop();
                        new_path.push(animation.json_path.clone());
                        fs::copy(previous_path, new_path);
                    }

                    if let EntityComponent::SpriteRenderer(sprite_renderer) = component
                    {   
                        let mut previous_path = json_path.clone();
                        previous_path.pop();
                        previous_path.push(sprite_renderer.sprite.clone());

                        let mut new_path = build_path.clone();
                        new_path.pop();
                        new_path.push(sprite_renderer.sprite.clone());
                        fs::copy(previous_path, new_path);
                    }
                }
            }

            let mut file = File::create(build_path).unwrap();
            file.write_all(&buf);
            
        }
    }
}



