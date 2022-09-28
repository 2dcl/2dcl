use bevy::prelude::*;
use bevy::sprite::Anchor;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Result;
use bevy_inspector_egui::Inspectable;
use super::collision::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use bevy::render::texture::ImageSampler;


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
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: EntityAnchor
}

#[derive(Deserialize, Serialize, Debug, Clone, Inspectable)]
pub enum EntityAnchor{
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    Custom(Vec2),
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
    pub anchor: EntityAnchor
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
    if let Ok(file) = File::open("./2dcl/assets/scene.json")
    {
        let reader = BufReader::new(file);
        let scene: Result<Scene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();

            for entity in  scene.entities.iter()
            {
                let mut transform = Transform::identity();

                for component in entity.components.iter()
                {
                    if let EntityComponent::Transform(entity_transform) = component
                    {       
                        transform.translation = transform.translation + entity_transform.location.extend(entity_transform.location.y * -1.0);
                        transform.rotation = Quat::from_euler(
                            EulerRot::XYZ,
                            entity_transform.rotation.x.to_radians(),
                            entity_transform.rotation.y.to_radians(),
                            entity_transform.rotation.z.to_radians());

                        transform.scale = entity_transform.scale.extend(1.0);
                    }  
                }
                
                //Spawning Entity
                let spawned_entity = commands.spawn()
                    .insert(Name::new(entity.name.clone()))
                    .insert(Visibility{is_visible: true})
                    .insert(GlobalTransform::default())
                    .insert(ComputedVisibility::default())
                    .insert(transform)
                    .id();
                //Inserting components
                for component in entity.components.iter()
                {
                    
                    if let EntityComponent::SpriteRenderer(sprite_renderer) = component
                    {
                        if let Ok(mut reader) = ImageReader::open("./2dcl/assets/".to_owned()+&sprite_renderer.sprite)
                        {
                            reader.set_format(ImageFormat::Png);
                            if let Ok(dynamic_image) = reader.decode()
                            {
                                if let DynamicImage::ImageRgba8(image) = dynamic_image
                                {
                                   
                                    let mut pixels = image.pixels().into_iter();
                                    let rows = image.rows().len();
                                    let columns = pixels.len()/rows;


                                    let texture: Handle<Image> = asset_server.load(&sprite_renderer.sprite);

                                    transform.translation = Vec3{
                                        x:transform.translation.x,
                                        y:transform.translation.y,
                                        z:transform.translation.z + sprite_renderer.layer as f32 * 500.0
                                    };
                                    let sprite = Sprite{
                                            color: Color::Rgba { 
                                                red: sprite_renderer.color.x, 
                                                green: sprite_renderer.color.y, 
                                                blue: sprite_renderer.color.z, 
                                                alpha:  sprite_renderer.color.w},
                                                anchor: entity_anchor_to_anchor(Vec2{x:columns as f32, y:rows as f32},sprite_renderer.anchor.clone()),
                                                flip_x: sprite_renderer.flip_x,
                                                flip_y: sprite_renderer.flip_y,
                                            ..default()
                                        };

                                        
                                    commands.entity(spawned_entity).insert(texture);
                                    commands.entity(spawned_entity).insert(transform);
                                    commands.entity(spawned_entity).insert(sprite);

                                }
                            }
                        }
                    }

                    if let EntityComponent::BoxCollider(collider) = component
                    {      
                        commands.entity(spawned_entity).insert(collider.clone());
                    }
    
                    if let EntityComponent::CircleCollider(collider) = component
                    {
                        commands.entity(spawned_entity).insert(collider.clone());
                    }
                    
                    
                    if let EntityComponent::AlphaCollider(collider) = component
                    {
                        if let Ok(mut reader) = ImageReader::open("./2dcl/assets/".to_owned()+&collider.sprite)
                        {        
                            reader.set_format(ImageFormat::Png);
                            if let Ok(dynamic_image) = reader.decode()
                            {
                                if let DynamicImage::ImageRgba8(image) = dynamic_image
                                {
                                    let mut pixels = image.pixels().into_iter();
                                    
                                    let rows = image.rows().len();
                                    let columns = pixels.len()/rows;

                                    let fixed_translation = get_fixed_translation_by_anchor(
                                        Vec2{x:columns as f32, y: rows as f32},
                                        transform.translation,
                                        collider.anchor.clone()
                                    );
                                    let mut index =0;
                                    let strating_world_location =
                                    
                                    fixed_translation.truncate() - (Vec2::new((columns as f32)/2.0 , (rows as f32)/2.0)* collision_map.tile_size);
                                     
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
        else
        {   
            println!("{:?}",scene);
        }
    }
}


fn entity_anchor_to_anchor(size: Vec2, anchor: EntityAnchor) -> Anchor
{
    match anchor
    {
        EntityAnchor::BottomCenter => return Anchor::BottomCenter,
        EntityAnchor::BottomLeft => return Anchor::BottomLeft,
        EntityAnchor::BottomRight => return Anchor::BottomRight,
        EntityAnchor::Center => return Anchor::Center,
        EntityAnchor::CenterLeft => return Anchor::CenterLeft,
        EntityAnchor::CenterRight => return Anchor::CenterRight,
        EntityAnchor::Custom(vec) => return Anchor::Custom(vec/size),
        EntityAnchor::TopCenter => return Anchor::TopCenter,
        EntityAnchor::TopLeft => return Anchor::TopLeft,
        EntityAnchor::TopRight => return Anchor::TopRight
    }
}


fn  get_fixed_translation_by_anchor(size: Vec2, translation: Vec3, anchor: EntityAnchor) -> Vec3
{

    match anchor
    {
        EntityAnchor::BottomCenter => return Vec3{x:translation.x, y:translation.y +size.y/2.0, z:translation.z},
        EntityAnchor::BottomLeft => return  Vec3{x:translation.x + size.x/2.0, y:translation.y +size.y/2.0, z:translation.z},
        EntityAnchor::BottomRight => return Vec3{x:translation.x - size.x/2.0, y:translation.y +size.y/2.0, z:translation.z},
        EntityAnchor::Center => return translation,
        EntityAnchor::CenterLeft => return Vec3{x:translation.x + size.x/2.0, y:translation.y, z:translation.z},
        EntityAnchor::CenterRight => return Vec3{x:translation.x - size.x/2.0, y:translation.y, z:translation.z},
        EntityAnchor::Custom(vec) => return Vec3{x:translation.x - vec.x, y:translation.y - vec.y, z:translation.z},
        EntityAnchor::TopCenter => return Vec3{x:translation.x, y:translation.y - size.y/2.0, z:translation.z},
        EntityAnchor::TopLeft => return Vec3{x:translation.x + size.x/2.0, y:translation.y -size.y/2.0, z:translation.z},
        EntityAnchor::TopRight => return  Vec3{x:translation.x - size.x/2.0, y:translation.y -size.y/2.0, z:translation.z},
    }
}