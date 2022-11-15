use std::time::SystemTime;
use std::fs;
use std::fs::create_dir;
use std::io::Write;
use std::str::FromStr;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::Task;

use catalyst::entity_files::ContentFile;
use catalyst::{ContentClient, Server};
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::{Parcel};
use bevy::sprite::Anchor;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use bevy_inspector_egui::Inspectable;
use std::path::PathBuf;

use super::dcl_3d_scene;
use super::player::PlayerComponent;
use futures_lite::future;
use rmp_serde::*;

use crate::renderer::config::*;

pub struct SceneLoaderPlugin;


#[derive(Component)]
pub struct DownloadingScene {
    pub task: Task<()>,
    pub parcels: Vec<Parcel>,
}


#[derive(Debug, Component, Clone, Inspectable)]
pub struct CircleCollider {
    pub center: Vec2,
    pub radius: u32,
}

#[derive(Debug, Component, Clone)]
pub struct BoxCollider {
    pub center: Vec2,
    pub size: Vec2,
    pub collision_type: CollisionType,
}

#[derive(Debug, Component, Clone)]
pub struct LevelChangeComponent {
  pub level: usize,
  pub spawn_point: Vec2,
}

#[derive(Debug, Component)]
pub struct SceneComponent
{
    pub name: String,
    pub parcels: Vec<Parcel>,
    pub timestamp: SystemTime,
    pub scene_data: Vec<u8>,
    pub path: PathBuf,
}

#[derive(Debug, Component, Clone)]
pub struct LevelComponent
{
    pub name: String,
    pub timestamp: SystemTime,
    pub id: usize,
}



impl Plugin for SceneLoaderPlugin
{

    fn build(&self, app: &mut App) {
    app
    .add_system(scene_handler)
    .add_system(handle_tasks)
    ;
    }
}



pub fn scene_handler(
    mut player_query: Query<(&mut PlayerComponent, &mut GlobalTransform)>,  
    scene_query: Query<(Entity, &mut SceneComponent, Without<PlayerComponent>)>,  
    level_query: Query<(Entity, &LevelComponent,&Parent)>,
    downloading_scenes_query: Query<&DownloadingScene>,  
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    
)
{
  //Find the player
  let player_query = player_query.get_single_mut();

  if player_query.is_err()
  {
    return;
  }
  

  let mut player_query = player_query.unwrap();
  let player_parcel = player_query.0.current_parcel.clone();
  
  
  let current_level = player_query.0.current_level;

  //We check if we're on the correct level
 
  for(scene_entity, scene, _player) in scene_query.iter()
  {       

    if  scene.parcels.contains(&player_parcel)
    {   
      for (level_entity, level, level_parent) in level_query.iter()
      {
  
        if **level_parent == scene_entity
        {

          if current_level != level.id
          {
            //Despawn level for current parcel
            commands.entity(level_entity).despawn_recursive();
        
            //Despawn every other scene and level
            for(other_scene_entity, _other_scene, _player) in scene_query.iter()
            {
              if other_scene_entity != scene_entity
              {
                commands.entity(other_scene_entity).despawn_recursive();
              }
            }

            //Spawn correct level
            let mut de = Deserializer::from_read_ref(&scene.scene_data);
            let scene_data: dcl2d_ecs_v1::Scene = Deserialize::deserialize(&mut de).unwrap();
            let level_entity = spawn_level(&mut commands,&asset_server,&scene_data,current_level,&scene.path,SystemTime::now());
            commands.entity(scene_entity).add_child(level_entity);
          }
          break;
        }
      
      }
      break;
    } 
  }

  //Only continue if we're in the overworld.
  if current_level != 0
  {  
    return;
  }

  let mut parcels_to_spawn = get_all_parcels_around(&player_parcel, MIN_RENDERING_DISTANCE_IN_PARCELS);
  let parcels_to_keep = get_all_parcels_around(&player_parcel, MAX_RENDERING_DISTANCE_IN_PARCELS);
  

  //Check every scene already spawned
  for(entity, scene, _player) in scene_query.iter()
  {       

    //Despawning scenes far away
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
        commands.entity(entity).despawn_recursive();
        continue;
    }

    //We don't need to spawn parcels already spawned 
    for i in (0..parcels_to_spawn.len()).rev()
    {
        if  scene.parcels.contains(&parcels_to_spawn[i])
        { 
          parcels_to_spawn.remove(i);
        } 
    }
  }

  //Spawning scenes
  let mut itr = parcels_to_spawn.len() as i16 -1;
  let mut parcels_to_download: Vec<Parcel> = Vec::default();
  while itr >=0
  {            
    
    //Check if it's already downloaded
    let result = get_scene(Parcel(parcels_to_spawn[itr as usize].0,parcels_to_spawn[itr as usize].1));
    let scene = result.0;
    let path = result.1;
    if scene.is_ok()
    {
      let scene = scene.unwrap();
      
      for scene_parcel in &scene.parcels
      {
          for i in (0..parcels_to_spawn.len()).rev()
          {
              if  parcels_to_spawn[i] == *scene_parcel
              {
                parcels_to_spawn.remove(i);
              }
          }
      }

      //If it's already downloaded, we spawn the scene.
      spawn_scene(&mut commands,&asset_server, scene,path,SystemTime::now());   
      continue;
    }
    else
    {

      //If the scene is already being downloaded we do nothing.
      let mut is_downloading = false;
      for downloading_scene in downloading_scenes_query.iter()
      {
          if downloading_scene.parcels.contains(&parcels_to_spawn[itr as usize])
          {          
              is_downloading = true;
              break;
          }

      }
      if !is_downloading
      {
        //We add the scene to download.
        parcels_to_download.push(parcels_to_spawn[itr as usize].clone());
      }
      parcels_to_spawn.remove(itr as usize);
        
    }
    itr = parcels_to_spawn.len() as i16 -1;
  }

  player_query.0.current_parcel = world_location_to_parcel(player_query.1.translation());
  

  if parcels_to_download.is_empty()
  {    
      return;
  }

        
  //We download the scenes needed
  let thread_pool = AsyncComputeTaskPool::get();
  let parcels_to_download_clone = parcels_to_download.clone();
  let task_download_parcels = thread_pool.spawn(async move {

    download_parcels(parcels_to_download_clone);

  }); 

  for parcel_to_download in &parcels_to_download
  {
      spawn_default_scene(&mut commands, &asset_server, parcel_to_download);
  } 

  commands.spawn().insert(DownloadingScene{task:task_download_parcels,parcels: parcels_to_download});


} 



fn get_scene_center_location(scene: &dcl2d_ecs_v1::Scene) -> Vec3
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

    let mut parcels: Vec<Parcel> =Vec::new();

    for x in 0..distance
    {
        for y in 0..distance
        {
     
            parcels.push(Parcel(parcel.0+x, parcel.1+y));

            if x!=0
            {
                parcels.push(Parcel(parcel.0-x, parcel.1+y));
            }

            if y!=0
            {
                parcels.push(Parcel(parcel.0+x, parcel.1-y));
            }
                
            if (x!=0) && (y!=0)
            {
                parcels.push(Parcel(parcel.0-x, parcel.1-y));
            }
        }
    }

    parcels
}
pub fn world_location_to_parcel(location: Vec3) -> Parcel
{
    return Parcel((location.x/PARCEL_SIZE_X).round() as i16,(location.y/PARCEL_SIZE_Y).round() as i16);
}

#[tokio::main]
pub async fn download_parcels(parcels: Vec<Parcel>) -> dcl_common::Result<()> {
    let server = Server::production();

    let scene_files = ContentClient::scene_files_for_parcels(&server, &parcels).await?;


    for scene_file in scene_files 
    {
        let path_str = "./assets/scenes/".to_string() + &scene_file.id.to_string();
        let scene_path = Path::new(&path_str);
        if !scene_path.exists()
        {   
            let mut file_2dcl_exists = false;
            let mut file_json: Option<ContentFile> = None;

            for downloadable in scene_file.clone().content 
            {
                if  downloadable.filename.to_str().unwrap().ends_with("scene.2dcl")
                {
                    file_2dcl_exists = true;
                    break;
                }
                if  downloadable.filename.to_str().unwrap().ends_with("scene.json")
                {
                    file_json = Some(downloadable);
                }
            }

            if !file_2dcl_exists && file_json.is_some()
            {
                let file_json = file_json.unwrap();
                let filename = format!(
                    "./assets/scenes/{}/{}",
                    scene_file.id,
                    file_json.filename.to_str().unwrap()
                );
                println!("Downloading {}", filename);
                ContentClient::download(&server, file_json.cid, &filename).await?;
             
                if let Ok(file) = File::open(filename)
                {
                    let reader = BufReader::new(file);
                    let scene: serde_json::Result<dcl_3d_scene::DCL3dScene> = serde_json::from_reader(reader);
                    
                    if scene.is_ok()
                    { 
                        println!("scene.is_ok()");
                        let scene = scene.unwrap();
                        if scene.display.title.to_lowercase().contains("road") || scene.display.title.to_lowercase().contains("tram line")
                        {
                            for parcel_in_scene in scene.scene.parcels
                            {
                                make_road_scene_for_parcel(&parcel_in_scene);
                            }
                        }
                    }
                }
            }

            if file_2dcl_exists
            {

                fs::create_dir_all(format!("./assets/scenes/{}", scene_file.id))?;
                for downloadable in scene_file.content {
                    let filename = format!(
                        "./assets/scenes/{}/{}",
                        scene_file.id,
                        downloadable.filename.to_str().unwrap()
                    );
                    println!("Downloading {}", filename);
                    ContentClient::download(&server, downloadable.cid, filename).await?;
                }
            }
        }
    }
    
    
    Ok(())
}


fn make_road_scene_for_parcel (parcel: &Parcel)
{
    let scene = read_scene_file("./assets/scenes/templates/road/scene.2dcl");

    if scene.is_some()
    {
        let mut scene = scene.unwrap();
        scene.parcels = vec![parcel.clone()];
      
        let mut buf: Vec<u8> = Vec::new();
        scene.serialize(&mut Serializer::new(&mut buf)).unwrap();   
        let save_path =  "./assets/scenes/road_".to_string() + &parcel.0.to_string() + "_" + &parcel.1.to_string();
        create_dir(&save_path);
        let mut file = File::create(save_path.clone() + "/scene.2dcl").unwrap();
        file.write_all(&buf); 

        
        let save_path = save_path + "/assets";
        create_dir(&save_path);
        
        fs::copy("./assets/scenes/templates/road/assets/background.png", save_path + "/background.png");
    }
}


pub fn read_scene(content: &[u8]) -> Option<dcl2d_ecs_v1::Scene>
{
        let mut de = Deserializer::new(content);
        let scene: Result<dcl2d_ecs_v1::Scene,rmp_serde::decode::Error> = Deserialize::deserialize(&mut de);
        if scene.is_ok()
        {
            return Some(scene.unwrap());
        }
        else
        {
            return None;
        }

}

pub fn read_scene_file <P>(file_path :P) -> Option<dcl2d_ecs_v1::Scene>
where P: AsRef<Path>
{

    if let Ok(file) = File::open(&file_path)
    {
        let reader = BufReader::new(file);
        let mut de = Deserializer::new(reader);
        let scene: Result<dcl2d_ecs_v1::Scene,rmp_serde::decode::Error> = Deserialize::deserialize(&mut de);
        if scene.is_ok()
        {
            return Some(scene.unwrap());
        }
        else
        {
            println!("scene is not ok");
            return None;
        }
    }
    else
    {
      println!("no path: {:?}",file_path.as_ref());
    }
    
    None
}

fn get_scene(parcel: Parcel) -> (Result<dcl2d_ecs_v1::Scene, String>, PathBuf)
{

    //TODO: map paths to scenes to improve performance.
    let paths = fs::read_dir("./assets/scenes").unwrap();

    for path in paths
    {    
        if path.is_ok()
        {
            let mut path = path.unwrap().path();
            path.push("scene.2dcl");
         
            if path.exists()
            {
                let scene = read_scene_file(&path);
                if scene.is_some()
                {
                    let scene = scene.unwrap();
                    if scene.parcels.contains(&parcel)
                    {   
                        let path = path.parent().unwrap().to_path_buf();
                        let iter = path.iter().rev();
                        let mut new_path = PathBuf::default();
                        for i in iter
                        {
                            new_path.push(i);
                        }
                        new_path.pop();
                        new_path.pop();
                        let iter = new_path.iter().rev();

                        let mut final_path = PathBuf::default();
                        for i in iter
                        {
                            final_path.push(i);
                        }

                        return (Ok(scene),final_path);
                    }
                }
            }
        }  
    }
    (Err("Parcel not downloaded".to_string()),PathBuf::default())


}


fn handle_tasks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tasks_downloading_scenes: Query<(Entity, &mut DownloadingScene)>,
    scenes_query: Query<(Entity, &SceneComponent)>,
) 
{ 

    for (entity, mut downloading_scene) in &mut tasks_downloading_scenes {
        if let Some(_finished) = future::block_on(future::poll_once(&mut downloading_scene.task)) 
        {
            commands.entity(entity).despawn_recursive();

            for parcel in &downloading_scene.parcels
            {
              let result =  get_scene(parcel.clone());

              if result.0.is_ok()
              {
                spawn_scene(&mut commands, &asset_server, result.0.unwrap(), result.1, SystemTime::now());
              }
            }
        }
    } 


    for (entity_1,scene_1) in &scenes_query
    {
        for (entity_2,scene_2) in &scenes_query
        {
            if entity_1 != entity_2 && (scene_1.name == "Sample Scene" || scene_2.name == "Sample Scene")
            {
                'outer: for parcel_1 in &scene_1.parcels
                {

                    for parcel_2 in &scene_2.parcels
                    {
                        if *parcel_1 == *parcel_2
                        {
                            if scene_1.name == "Sample Scene"
                            {
                                println!("Despawning empty_parcel {:?}", parcel_1);
                                commands.entity(entity_1).despawn_recursive();
                                break 'outer
                            }

                            if scene_2.name == "Sample Scene"
                            {
                                println!("Despawning empty_parcel {:?}", parcel_2);
                                commands.entity(entity_2).despawn_recursive();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}



fn spawn_default_scene(    
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parcel: &Parcel,
)
{

  let mut scene = read_scene_file("./assets/scenes/templates/empty_parcel/scene.2dcl").unwrap();
  scene.parcels = vec![parcel.clone()];
  spawn_scene(commands, asset_server, scene,"./scenes/templates/empty_parcel",SystemTime::now());
}

pub fn spawn_level<T>( 
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene: &dcl2d_ecs_v1::Scene,
    level_id: usize,
    path: T,
    timestamp: SystemTime
) -> Entity
where
T: AsRef<Path>
{
  let scene_entity = commands.spawn().id();

  if scene.levels.len()<=level_id
  {
    return scene_entity;
  }

  let level = &scene.levels[level_id];
   
    commands.entity(scene_entity)
    .insert(Name::new(level.name.clone()))
    .insert(Visibility{is_visible: true})
    .insert(GlobalTransform::default())
    .insert(ComputedVisibility::default())
    .insert(Transform::default())
    .insert(LevelComponent{name:level.name.clone(),timestamp,id: level_id});
    
    
    
    for entity in  level.entities.iter()
    {
        let mut transform = Transform::identity();

        for component in entity.components.iter()
        {
            if let Some(source_transform) = component.as_any().downcast_ref::<dcl2d_ecs_v1::components::Transform>() {
                let location = Vec2::new(source_transform.location.x as f32, source_transform.location.y as f32);
                let scale = Vec2::new(source_transform.scale.x, source_transform.scale.y);

                transform.translation = transform.translation + location.extend(location.y * -1.0);
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    source_transform.rotation.x.to_radians(),
                    source_transform.rotation.y.to_radians(),
                    source_transform.rotation.z.to_radians());

                transform.scale = scale.extend(1.0);
            };
        }
        
        //Spawning Entity
        let spawned_entity = commands.spawn()
            .insert(Name::new(entity.name.clone()))
            .insert(Visibility{is_visible: true})
            .insert(GlobalTransform::default())
            .insert(ComputedVisibility::default())
            .insert(transform)
            .id();
       
        commands.entity(scene_entity).add_child(spawned_entity);
        
        // Inserting components
        for component in entity.components.iter()
        { 
            if let Some(sprite_renderer) = component.as_any().downcast_ref::<dcl2d_ecs_v1::components::SpriteRenderer>() {
                transform.translation = Vec3{
                    x:transform.translation.x,
                    y:transform.translation.y,
                    z:transform.translation.z + sprite_renderer.layer as f32 * 500.0
                };

                commands.entity(spawned_entity).insert(transform);

                let server = (*asset_server).clone();
                let mut image_path = path.as_ref().clone().to_path_buf();
                image_path.push("assets");
                image_path.push(&sprite_renderer.sprite);

                let texture: Handle<Image> = server.load(image_path);
                commands.entity(spawned_entity).insert(texture);
              
                let renderer  = (*sprite_renderer).clone();
                let mut sprite_path = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
                sprite_path.push("assets");
                sprite_path.push(path.as_ref().clone());
                sprite_path.push("assets");
                sprite_path.push(&sprite_renderer.sprite);

                let sprite = Sprite{
                  color: Color::Rgba { 
                      red: renderer.color.r, 
                      green: renderer.color.g, 
                      blue: renderer.color.b, 
                      alpha:  renderer.color.a
                  },
                  anchor: entity_anchor_to_anchor(renderer.anchor.clone()),
                  flip_x: renderer.flip.x,
                  flip_y: renderer.flip.y,
                  ..default()
              };

              commands.entity(spawned_entity).insert(sprite);
            }

            if let Some(collider) = component.as_any().downcast_ref::<dcl2d_ecs_v1::components::BoxCollider>() {

                let box_collider= BoxCollider{
                  center:Vec2::new(collider.center.x as f32, collider.center.y as f32),
                  size:Vec2::new(collider.size.width as f32, collider.size.height as f32),
                  collision_type: collider.collision_type.clone()
                };
                commands.entity(spawned_entity).insert(box_collider);
            }

             if let Some(collider) = component.as_any().downcast_ref::<dcl2d_ecs_v1::components::CircleCollider>() {                      
                let circle_collider = CircleCollider{center:Vec2::new(collider.center.x as f32,collider.center.y as f32),radius:collider.radius};
                commands.entity(spawned_entity).insert(circle_collider);
            }


            if let Some(level_change) = component.as_any().downcast_ref::<dcl2d_ecs_v1::components::triggers::LevelChange>() {

              let mut new_level_id = 0;

              for i in 0..scene.levels.len()
              {
                if scene.levels[i].name == level_change.level
                {
                  new_level_id = i;
                  break;
                }
              }

              let level_change_component = LevelChangeComponent {
                level:new_level_id,
                spawn_point: Vec2::new(level_change.spawn_point.x as f32, level_change.spawn_point.y as f32),
              };
              commands.entity(spawned_entity).insert(level_change_component);
     
          }
        }
    } 
    scene_entity
}


pub fn spawn_scene<T>(    
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene: dcl2d_ecs_v1::Scene,
    path: T,
    timestamp: SystemTime
) -> Entity
where T: AsRef<Path>
{
  

  let scene_location: Vec3 = get_scene_center_location(&scene);
  let mut scene_data: Vec<u8> = Vec::new();
  scene.serialize(&mut Serializer::new(&mut scene_data)).unwrap();
  let scene_entity = commands.spawn()
  .insert(Visibility{is_visible: true})
  .insert(GlobalTransform::default())
  .insert(ComputedVisibility::default())
  .insert(Name::new(scene.name.clone()))
  .insert(Transform::from_translation(scene_location))
  .insert(SceneComponent{name:scene.name.clone(),parcels:scene.parcels.clone(), timestamp, scene_data,path: path.as_ref().to_path_buf()})
  .id();

  if scene.levels.len()>0
  {
    let level_entity = spawn_level(commands,asset_server,&scene,0,path,SystemTime::now());
    commands.entity(scene_entity).add_child(level_entity);
  }
  

  scene_entity

}

fn entity_anchor_to_anchor(anchor: dcl2d_ecs_v1::Anchor) -> Anchor
{
    match anchor
    {
        dcl2d_ecs_v1::Anchor::BottomCenter => return Anchor::BottomCenter,
        dcl2d_ecs_v1::Anchor::BottomLeft => return Anchor::BottomLeft,
        dcl2d_ecs_v1::Anchor::BottomRight => return Anchor::BottomRight,
        dcl2d_ecs_v1::Anchor::Center => return Anchor::Center,
        dcl2d_ecs_v1::Anchor::CenterLeft => return Anchor::CenterLeft,
        dcl2d_ecs_v1::Anchor::CenterRight => return Anchor::CenterRight,
        // dcl2d_ecs_v1::Anchor::Custom(vec) => return Anchor::Custom(Vec2::new(vec.0, vec.1)/size),
        dcl2d_ecs_v1::Anchor::TopCenter => return Anchor::TopCenter,
        dcl2d_ecs_v1::Anchor::TopLeft => return Anchor::TopLeft,
        dcl2d_ecs_v1::Anchor::TopRight => return Anchor::TopRight,
        dcl2d_ecs_v1::Anchor::Custom(_) => todo!()
    }
}

/*
fn  get_fixed_translation_by_anchor(size: Vec2, translation: Vec3, anchor: dcl2d_ecs_v1::Anchor) -> Vec3
{

    match anchor
    {
        dcl2d_ecs_v1::Anchor::BottomCenter => return Vec3{x:translation.x, y:translation.y +size.y/2.0, z:translation.z},
        dcl2d_ecs_v1::Anchor::BottomLeft => return  Vec3{x:translation.x + size.x/2.0, y:translation.y +size.y/2.0, z:translation.z},
        dcl2d_ecs_v1::Anchor::BottomRight => return Vec3{x:translation.x - size.x/2.0, y:translation.y +size.y/2.0, z:translation.z},
        dcl2d_ecs_v1::Anchor::Center => return translation,
        dcl2d_ecs_v1::Anchor::CenterLeft => return Vec3{x:translation.x + size.x/2.0, y:translation.y, z:translation.z},
        dcl2d_ecs_v1::Anchor::CenterRight => return Vec3{x:translation.x - size.x/2.0, y:translation.y, z:translation.z},
        // dcl2d_ecs_v1::Anchor::Custom(vec) => return Vec3{x:translation.x - vec.0, y:translation.y - vec.1, z:translation.z},
        dcl2d_ecs_v1::Anchor::TopCenter => return Vec3{x:translation.x, y:translation.y - size.y/2.0, z:translation.z},
        dcl2d_ecs_v1::Anchor::TopLeft => return Vec3{x:translation.x + size.x/2.0, y:translation.y -size.y/2.0, z:translation.z},
        dcl2d_ecs_v1::Anchor::TopRight => return  Vec3{x:translation.x - size.x/2.0, y:translation.y -size.y/2.0, z:translation.z},
        dcl2d_ecs_v1::Anchor::Custom(_) => todo!(),
    }
} */

