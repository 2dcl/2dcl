use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Result;
use bevy_inspector_egui::Inspectable;
use super::collision::*;
use image::io::Reader as ImageReader;
use image::DynamicImage;


#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
   pub name: String,
   pub entities: Vec<Entitiy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entitiy {
    pub name: String,
    pub components: Vec<EntityComponent>,
}


#[derive(Deserialize, Serialize, Debug)]
pub enum EntityComponent{
    Transform(EntityTransform),
    SpriteRenderer(SpriteRenderer),
    CircleCollider(CircleCollider),
    BoxCollider(BoxCollider),
    AlphaCollider(AlphaCollider)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityTransform {
    pub location: Vec2,
    pub rotation: Vec3,
    pub scale: Vec2,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpriteRenderer {
    pub sprite: String,
    pub color: Vec4,
    pub layer: i32,
}

#[derive(Deserialize, Serialize, Debug, Component, Clone, Inspectable)]
pub struct CircleCollider {
    pub center: Vec2,
    pub raius: i32,
}

#[derive(Deserialize, Serialize, Debug, Component, Clone, Inspectable)]
pub struct BoxCollider {
    pub center: Vec2,
    pub size: Vec2,
}

#[derive(Deserialize, Serialize, Debug, Clone, Inspectable)]
pub struct AlphaCollider {
    pub sprite: String,
    pub channel: i32,
}


pub struct SceneDeserializerPlugin;


impl Plugin for SceneDeserializerPlugin
{
    fn build(&self, app: &mut App)
    {
         app.add_startup_system(load_scene);
    }

}

pub fn save_scene <P>(scene: Scene, path: P)
where
    P: AsRef<Path>
{
    let writer;

    match File::create(path)
    {
        Ok(v) => writer = v,
        Err(_e) => return
    } 



    match serde_json::to_writer(&writer, &scene)
    {
        Ok(_v) => println!("saved json scene"),
        Err(_e) => return
    }
    
}


fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_map: ResMut<CollisionMap>

) {
    if let Ok(file) = File::open("./assets/scene.json")
    {
        let reader = BufReader::new(file);
        let scene: Result<Scene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();

            for entity in  scene.entities.iter()
            {
                let mut transform = Transform::identity();
                let mut texture: Handle<Image> = Handle::default();
                let mut sprite = Sprite::default();
                
                //Finding entity paramters
               for component in entity.components.iter()
               {
                
                if let EntityComponent::Transform(entity_transform) = component
                {

                    transform.translation = transform.translation + entity_transform.location.extend(0.0);
                    transform.rotation = Quat::from_euler(
                        EulerRot::XYZ,
                        entity_transform.rotation.x.to_radians(),
                        entity_transform.rotation.y.to_radians(),
                        entity_transform.rotation.z.to_radians());

                    transform.scale = entity_transform.scale.extend(1.0);
                }

                if let EntityComponent::SpriteRenderer(sprite_renderer) = component
                {
                    sprite.color = Color::Rgba { 
                        red: sprite_renderer.color.x, 
                        green: sprite_renderer.color.y, 
                        blue: sprite_renderer.color.z, 
                        alpha:  sprite_renderer.color.w};
                    texture = asset_server.load(&sprite_renderer.sprite);
                    transform.translation = transform.translation + Vec3::new(0.0,0.0, sprite_renderer.layer as f32);
                }


                
               }
               
                //Spawning Entity
               let mut spawn_bundle = commands.spawn_bundle(SpriteBundle {
                transform,
                sprite: sprite.clone(),
                texture: texture.clone(),
                ..default()
                });
                
                spawn_bundle.insert(Name::new(entity.name.clone()));

                //Inserting components
                for component in entity.components.iter()
                {
                    if let EntityComponent::BoxCollider(collider) = component
                    {  
                        spawn_bundle.insert(collider.clone());
                    }
    
                    if let EntityComponent::CircleCollider(collider) = component
                    {
                        spawn_bundle.insert(collider.clone());
                    }
    
                    
                    if let EntityComponent::AlphaCollider(collider) = component
                    {
                        if let Ok(reader) = ImageReader::open("./assets/".to_owned()+&collider.sprite)
                        {
                            if let Ok(dynamic_image) = reader.decode()
                            {
                                if let DynamicImage::ImageRgba8(image) = dynamic_image
                                {
                                    let mut pixels = image.pixels().into_iter();

                                    let rows = image.rows().len();
                                    let columns = pixels.len()/rows;
                                    let mut index =0;
                                    let strating_world_location =
                                    
                                    transform.translation.truncate() - (Vec2::new((columns as f32)/2.0 , (rows as f32)/2.0)* collision_map.tile_size);
                                    

                                    
                                    while pixels.len() >0
                                    {   
                                        if pixels.next().unwrap()[collider.channel as usize] > 0 
                                        {
                                            let world_location = strating_world_location + (Vec2::new((index%columns) as f32,(index/columns) as f32)*collision_map.tile_size);
                                            collision_map.collision_locations.push(world_location);
                                        }                             
                                        index +=1;
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
