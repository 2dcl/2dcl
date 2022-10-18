use std::fs;
use bevy::prelude::*;
use catalyst::{ContentClient, Server};
use dcl_common::{Parcel, Result};
use bevy::sprite::Anchor;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use bevy_inspector_egui::Inspectable;
use super::collision::*;
use super::animations::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use super::player::Player;
use walkdir::WalkDir;

pub struct SceneLoaderPlugin;


pub const MIN_RENDERING_DISTANCE_IN_PARCELS: i16 = 3;
pub const MAX_RENDERING_DISTANCE_IN_PARCELS: i16 = 4;
pub const PARCEL_SIZE_X: f32 = 350.0;
pub const PARCEL_SIZE_Y: f32 = 350.0;

#[derive(Debug)]
pub struct Scene {
   pub name: String,
   pub entities: Vec<SceneEntity>,
   pub parcels: Vec<Parcel>,
   pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonScene {
   pub name: String,
   pub entities: Vec<SceneEntity>,
   pub parcels: Vec<Parcel>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneEntity {
    pub name: String,
    pub components: Vec<EntityComponent>,
}


#[derive(Deserialize, Serialize, Debug)]
pub enum EntityComponent{
    Transform(EntityTransform),
    SpriteRenderer(SpriteRenderer),
    CircleCollider(CircleCollider),
    BoxCollider(BoxCollider),
    AlphaCollider(AlphaCollider),
    AsepriteAnimation(AsepriteAnimation),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AsepriteAnimation {
    pub json_path: String,
    pub starting_state: String,
    pub color: Vec4,
    pub layer: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: EntityAnchor
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

#[derive(Debug, Component)]
pub struct SceneComponent
{
    pub name: String,
    pub parcels: Vec<Parcel>
}

impl Plugin for SceneLoaderPlugin
{

    fn build(&self, app: &mut App) {
    app
    .add_system(check_scenes_to_download)
    ;
    }
}

pub fn check_scenes_to_download(
    mut player_query: Query<(&mut Player, &mut GlobalTransform)>,  
    mut scene_query: Query<(Entity, &mut SceneComponent, Without<Player>)>,  
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_map: ResMut<CollisionMap>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{

    for(player,mut player_transform) in player_query.iter()
    {

        let player_parcel = world_location_to_parcel(player_transform.translation());
        let mut parcels_to_render = get_all_parcels_around(&player_parcel,MIN_RENDERING_DISTANCE_IN_PARCELS);
        
        let mut parcels_to_keep = get_all_parcels_around(&player_parcel,MAX_RENDERING_DISTANCE_IN_PARCELS);
       

        for(entity, scene, _player) in scene_query.iter()
        {       

            let mut despawn_scene = true;

            for parcel in &parcels_to_keep
            {
                if  scene.parcels.contains(&parcel)
                {   
                    despawn_scene = false;
                    break;
                } 
            }

          
            if despawn_scene
            {  
                println!("despawning scene");
                commands.entity(entity).despawn();
                continue;
            }


            for i in (0..parcels_to_render.len()).rev()
            {
                if  scene.parcels.contains(&parcels_to_render[i])
                {
                    parcels_to_render.remove(i);
                } 
            }
            
            if parcels_to_render.is_empty()
            {
                break;
            }
        }

        while !parcels_to_render.is_empty()
        {
            let scene = get_scene(Parcel(parcels_to_render[0].0,parcels_to_render[0].1));
            
            for scene_parcel in &scene.parcels
            {
                for i in (0..parcels_to_render.len()).rev()
                {
                    if  parcels_to_render[i] == *scene_parcel
                    {
                        parcels_to_render.remove(i);
                    }
                }
            }

            spawn_scene(&mut commands,&asset_server, &mut collision_map, &mut texture_atlases, scene);
            
      
          
        }
        break;
    }

}


fn get_scene_center_location(scene: &Scene) -> Vec3
{
 
    let mut min: Vec2 = Vec2{x:f32::MAX,y:f32::MAX};
    let mut max: Vec2 = Vec2{x:f32::MIN,y:f32::MIN};
 
    for parcel in &scene.parcels
    {
        if (parcel.0 as f32 * PARCEL_SIZE_X) < min.x
        {
            min.x = parcel.0 as f32 * PARCEL_SIZE_X;
        }

        if (parcel.1 as f32 * PARCEL_SIZE_Y) < min.y
        {
            min.y = parcel.1 as f32 * PARCEL_SIZE_Y;
        }
     
        if (parcel.0 as f32 * PARCEL_SIZE_X) > max.x
        {
            max.x = parcel.0 as f32 * PARCEL_SIZE_X;
        }

        if (parcel.1 as f32 * PARCEL_SIZE_Y) > max.y
        {
            max.y = parcel.1 as f32 * PARCEL_SIZE_Y;
        }
    }

    Vec3{x:(min.x+max.x)/2f32,y:(min.y+max.y)/2f32,z:(min.y+max.y)/-2f32}

}

fn get_all_parcels_around(parcel: &Parcel, distance: i16) -> Vec<Parcel>
{
    let mut parcels: Vec<Parcel> = Vec::default();

    for x in 0..distance
    {
        for y in 0..distance
        {
            parcels.push(Parcel(parcel.0+x, parcel.1+y));
        }
    }

    parcels
}
pub fn world_location_to_parcel(location: Vec3) -> Parcel
{
    return Parcel((location.x/PARCEL_SIZE_X).round() as i16,(location.y/PARCEL_SIZE_Y).round() as i16);
}

#[tokio::main]
pub async fn download_parcel(parcel: Parcel) -> Result<()> {
    let server = Server::production();

    let scene_files = ContentClient::scene_files_for_parcels(&server, &vec![parcel]).await?;

    for scene_file in scene_files {
        fs::create_dir_all(format!("./2dcl/assets/scenes/{}", scene_file.id))?;

        for downloadable in scene_file.content {
            let filename = format!(
                "./2dcl/assets/scenes/{}/{}",
                scene_file.id,
                downloadable.filename.to_str().unwrap()
            );
            println!("Downloading {}", filename);

            // We're downloading this synchronously, in a production client you want to
            // store all of these and use `futures::join_all` (https://docs.rs/futures/latest/futures/future/fn.join_all.html)
            // or something of the sorts.
            ContentClient::download(&server, downloadable.cid, filename).await?;
        }
    }
    println!("finished downloading");
    Ok(())
}
/*
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
    
} */

fn get_scene(parcel: Parcel) -> Scene
{
    for entry in WalkDir::new("./2dcl/assets/scenes") {
        let dir_entry =  entry.unwrap();
        if dir_entry.clone().file_name() == "scene.json"
        {
            if let Ok(file) = File::open( dir_entry.clone().path())
            {
                let reader = BufReader::new(file);
                let scene: serde_json::Result<JsonScene> = serde_json::from_reader(reader);
                
                if scene.is_ok()
                {
                    let mut scene = scene.unwrap();
                    if scene.parcels.contains(&parcel)
                    {   
                        let path = dir_entry.clone().path().parent().unwrap().to_str().unwrap().to_string();
                        return Scene{name:scene.name,
                            entities:scene.entities,
                            parcels:scene.parcels,
                            path:path};
                    }
                }
            }   
        }
    }
    

    download_parcel(Parcel(parcel.0,parcel.1));

    //TODO: we could get the path instead of searching for it again
    for entry in WalkDir::new("./assets/scenes") {
        let dir_entry =  entry.unwrap();
        if dir_entry.clone().file_name() == "scene.json"
        {
            if let Ok(file) = File::open( dir_entry.clone().path())
            {
                let reader = BufReader::new(file);
                let scene: serde_json::Result<JsonScene> = serde_json::from_reader(reader);
                
                if scene.is_ok()
                {
                    let mut scene = scene.unwrap();
                    if scene.parcels.contains(&parcel)
                    {   
                        let path = dir_entry.clone().path().parent().unwrap().to_str().unwrap().to_string();
                        return Scene{name:scene.name,
                            entities:scene.entities,
                            parcels:scene.parcels,
                            path:path};
                    }
                }
            }   
        }
    }

    //TODO error handling
    return Scene{name:"".to_string(),entities:Vec::default(),parcels:Vec::default(), path: "".to_string()};
  

}
/*
pub fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_map: ResMut<CollisionMap>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    scene_name: &str,
) {
    if let Ok(file) = File::open("./2dcl/assets/scenes/".to_string() + scene_name + "/scene.json")
    {
        let reader = BufReader::new(file);
        let scene: serde_json::Result<JsonScene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();
            spawn_scene(commands,asset_server,collision_map,texture_atlases,Scene { name: scene.name, entities: scene.entities, parcels: scene.parcels, path: () });
        }
    }
} */

fn spawn_scene(    
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    collision_map: &mut ResMut<CollisionMap>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    scene: Scene,
)
{

    let scene_location: Vec3 = get_scene_center_location(&scene);

    let scene_entity = commands.spawn()
    .insert(Name::new(scene.name.clone()))
    .insert(Visibility{is_visible: true})
    .insert(GlobalTransform::default())
    .insert(ComputedVisibility::default())
    .insert(Transform::from_translation(scene_location))
    .insert(SceneComponent{name:scene.name.clone(),parcels:scene.parcels})
    .id();

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
    println!("spawning entity: {:?}",entity.name.clone());
    commands.entity(scene_entity).add_child(spawned_entity);
    //Inserting components
    for component in entity.components.iter()
    {
        
        if let EntityComponent::SpriteRenderer(sprite_renderer) = component
        {
            println!("{:?}",scene.path.clone());
            if let Ok(mut reader) = ImageReader::open(scene.path.clone() + "/" + &sprite_renderer.sprite)
            {
                reader.set_format(ImageFormat::Png);
                if let Ok(dynamic_image) = reader.decode()
                {
                    if let DynamicImage::ImageRgba8(image) = dynamic_image
                    {
                       
                        let mut pixels = image.pixels().into_iter();
                        let rows = image.rows().len();
                        let columns = pixels.len()/rows;


                        let texture: Handle<Image> = asset_server.load(&("../../".to_string() + &scene.path.clone()  + "/" + &sprite_renderer.sprite));

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

        if let EntityComponent::AsepriteAnimation(aseprite_animation) = component
        {
           let mut animator =  get_animator(scene.path.clone()+ "/" + &aseprite_animation.json_path, &asset_server,&mut texture_atlases).unwrap();
           let sprite = TextureAtlasSprite
           {
            color: Color::Rgba { 
                red: aseprite_animation.color.x, 
                green: aseprite_animation.color.y, 
                blue: aseprite_animation.color.z, 
                alpha:  aseprite_animation.color.w},
                anchor: entity_anchor_to_anchor(Vec2{x:1 as f32, y:1 as f32},aseprite_animation.anchor.clone()),
                flip_x: aseprite_animation.flip_x,
                flip_y: aseprite_animation.flip_y,
            ..default()
            };
            change_animator_state(animator.borrow_mut(),aseprite_animation.starting_state.clone()); 
            commands.entity(spawned_entity).insert(sprite);
            commands.entity(spawned_entity).insert(animator.atlas.clone());
            commands.entity(spawned_entity).insert(animator);
        }
        
        
        
        if let EntityComponent::AlphaCollider(collider) = component
        {
            if let Ok(mut reader) = ImageReader::open(scene.path.clone() + "/" + &collider.sprite)
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
                            Vec3 { x: transform.translation.x + scene_location.x, 
                                y: transform.translation.y + scene_location.y, 
                                z: transform.translation.z + scene_location.z },
                            collider.anchor.clone(),
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