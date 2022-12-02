use crate::components::*;
use crate::renderer::road_maker::{make_road_scene, add_road_at_parcel, remove_road_at_parcel, is_road};

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::fs;
use std::fs::create_dir;
use std::io::Write;
use std::str::FromStr;
use std::time::SystemTime;

use catalyst::entity_files::ContentFile;
use catalyst::{ContentClient, Server};

use bevy::sprite::Anchor;
use dcl2d_ecs_v1::components::SpriteRenderer;
use dcl_common::Parcel;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::path::PathBuf;

use super::collision::CollisionMap;
use super::collision::CollisionTile;
use super::dcl_3d_scene;
use super::scenes_io::{ParcelMap, get_parcel_file, read_scene_file, add_parcel_file};
use super::player::PlayerComponent;
use super::road_maker::RoadsData;
use futures_lite::future;
use rmp_serde::*;

use crate::renderer::config::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use std::collections::HashMap;
use imagesize::size;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(scene_handler)
        .add_system(handle_tasks);
    }
}

pub fn scene_handler(
    mut player_query: Query<(&mut PlayerComponent, &mut GlobalTransform)>,
    scene_query: Query<(
        Entity,
        &mut crate::components::Scene,
        Without<PlayerComponent>,
    )>,
    level_query: Query<(Entity, &crate::components::Level, &Parent)>,
    downloading_scenes_query: Query<&DownloadingScene>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut roads_data: ResMut<RoadsData>,
    parcel_map: Res<ParcelMap>,
    mut collision_map: ResMut<CollisionMap>,
) {
    //Find the player
    let player_query = player_query.get_single_mut();

    if player_query.is_err() {
        return;
    }

    let mut player_query = player_query.unwrap();
    let player_parcel = player_query.0.current_parcel.clone();
    let current_level = player_query.0.current_level;

    player_query.0.current_parcel = world_location_to_parcel(&player_query.1.translation());
  
    //We check if we're on the correct level

    for (scene_entity, scene, _player) in scene_query.iter() {
        if scene.parcels.contains(&player_parcel) {
            for (level_entity, level, level_parent) in level_query.iter() {
                if **level_parent == scene_entity {
                    //If we're in a different level we change it
                    if current_level != level.id {
                        //Despawn level for current parcel
                        commands.entity(level_entity).despawn_recursive();

                        //Despawn every other scene and level
                        for (other_scene_entity, _other_scene, _player) in scene_query.iter() {
                            if other_scene_entity != scene_entity {
                                commands.entity(other_scene_entity).despawn_recursive();
                            }
                        }

                        //Clear collision map
                        collision_map.tiles.clear();

                        //Spawn correct level
                        let mut de = Deserializer::from_read_ref(&scene.scene_data);
                        let scene_data: dcl2d_ecs_v1::Scene =
                            Deserialize::deserialize(&mut de).unwrap();

                        let level_entity = spawn_level(
                            &mut commands,
                            &asset_server,
                            &scene_data,
                            current_level,
                            &scene.path,
                            &mut collision_map,
                            SystemTime::now(),
                        );
                        commands.entity(scene_entity).add_child(level_entity);
                    }
                    break;
                }
            }
            break;
        }
    }

    //Only continue if we're in the overworld.
    if current_level != 0 {
        return;
    }

    let mut parcels_to_spawn =
        get_all_parcels_around(&player_parcel, MIN_RENDERING_DISTANCE_IN_PARCELS);
        let parcels_to_keep = get_all_parcels_around(&player_parcel, MAX_RENDERING_DISTANCE_IN_PARCELS);

    //Check every scene already spawned
    for (entity, scene, _player) in scene_query.iter() {
        //Despawning scenes far away
        let mut despawn_scene = true;

        for parcel in &parcels_to_keep {
            if scene.parcels.contains(parcel) {
                despawn_scene = false;
                break;
            }
        }

        if despawn_scene {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        //We don't need to spawn parcels already spawned
        for i in (0..parcels_to_spawn.len()).rev() {
            if scene.parcels.contains(&parcels_to_spawn[i]) {
                parcels_to_spawn.remove(i);
            }
        }
    }
 
    if parcels_to_spawn.is_empty()
    {
      return;
    }
   
    //Spawning scenes
    let mut itr = parcels_to_spawn.len() as usize - 1;
    let mut parcels_to_download: Vec<Parcel> = Vec::default();
    while itr<parcels_to_spawn.len() {
        //Check if it's already downloaded
        let result = get_scene(
          &mut roads_data,
          &parcel_map,
          &Parcel(
            parcels_to_spawn[itr].0,
            parcels_to_spawn[itr].1,
        ));

        let path = result.1;

        if let Ok(scene) = result.0 {
            for scene_parcel in &scene.parcels {
                for i in (0..parcels_to_spawn.len()).rev() {
                    if parcels_to_spawn[i] == *scene_parcel {
                        parcels_to_spawn.remove(i);
                    }
                }
            }
            
            //If it's already downloaded, we spawn the scene.
            spawn_scene(
                &mut commands,
                &asset_server,
                scene,
                path,
                &mut collision_map,
                SystemTime::now(),
                0,
            );
            continue;
        } else {
            //If the scene is already being downloaded we do nothing.
            let mut is_downloading = false;
            for downloading_scene in downloading_scenes_query.iter() {
                if downloading_scene
                    .parcels
                    .contains(&parcels_to_spawn[itr as usize])
                {
                    is_downloading = true;
                    break;
                }
            }
            if !is_downloading {
                //We add the scene to download.
                parcels_to_download.push(parcels_to_spawn[itr as usize].clone());
            }
            parcels_to_spawn.remove(itr as usize);
        }
        
        if parcels_to_spawn.len()>0
        {
          itr = parcels_to_spawn.len() - 1;
        }
    }

    if parcels_to_download.is_empty() {
        return;
    }

    //We download the scenes needed
    let thread_pool = AsyncComputeTaskPool::get();
    let parcels_to_download_clone = parcels_to_download.clone();
    let task_download_parcels = thread_pool.spawn(async move {
        match download_parcels(parcels_to_download_clone)
        {
          Ok(v) => Some(v),
          Err(e) => {
            println!("{:?}",e);
            None
          }
        }
    });

    for parcel_to_download in &parcels_to_download {
        spawn_default_scene(
            &mut commands,
            &asset_server,
            parcel_to_download,
            &mut collision_map,
        );
    }

    commands.spawn().insert(DownloadingScene {
        task: task_download_parcels,
        parcels: parcels_to_download,
    });
}

fn get_scene_center_location(scene: &dcl2d_ecs_v1::Scene) -> Vec3 {
    let mut min: Vec2 = Vec2 {
        x: f32::MAX,
        y: f32::MAX,
    };
    let mut max: Vec2 = Vec2 {
        x: f32::MIN,
        y: f32::MIN,
    };

    for parcel in &scene.parcels {
        if (parcel.0 as f32 * PARCEL_SIZE_X) < min.x {
            min.x = parcel.0 as f32 * PARCEL_SIZE_X;
        }

        if (parcel.1 as f32 * PARCEL_SIZE_Y) < min.y {
            min.y = parcel.1 as f32 * PARCEL_SIZE_Y;
        }

        if (parcel.0 as f32 * PARCEL_SIZE_X) > max.x {
            max.x = parcel.0 as f32 * PARCEL_SIZE_X;
        }

        if (parcel.1 as f32 * PARCEL_SIZE_Y) > max.y {
            max.y = parcel.1 as f32 * PARCEL_SIZE_Y;
        }
    }

    Vec3 {
        x: (min.x + max.x) / 2f32,
        y: (min.y + max.y) / 2f32,
        z: (min.y + max.y) / -2f32,
    }
}

fn get_all_parcels_around(parcel: &Parcel, distance: i16) -> Vec<Parcel> {
    let mut parcels: Vec<Parcel> = Vec::new();

    for x in 0..distance {
        for y in 0..distance {
            parcels.push(Parcel(parcel.0 + x, parcel.1 + y));

            if x != 0 {
                parcels.push(Parcel(parcel.0 - x, parcel.1 + y));
            }

            if y != 0 {
                parcels.push(Parcel(parcel.0 + x, parcel.1 - y));
            }

            if (x != 0) && (y != 0) {
                parcels.push(Parcel(parcel.0 - x, parcel.1 - y));
            }
        }
    }

    parcels
}
pub fn world_location_to_parcel(location: &Vec3) -> Parcel {
    Parcel(
        (location.x / PARCEL_SIZE_X).round() as i16,
        (location.y / PARCEL_SIZE_Y).round() as i16,
    )
}

pub fn parcel_to_world_location(parcel: &Parcel) -> Vec3 {
  Vec3::new(parcel.0 as f32 * PARCEL_SIZE_X, 
    parcel.1 as f32 * PARCEL_SIZE_Y ,
    parcel.1 as f32 * PARCEL_SIZE_Y * -1.0)
}


#[tokio::main]
pub async fn download_parcels(parcels: Vec<Parcel>) -> dcl_common::Result<Vec<(PathBuf, Vec<Parcel>)>> {

  let server = Server::production();

  let mut parel_map: Vec<(PathBuf, Vec<Parcel>)> = Vec::new();

  let scene_files = ContentClient::scene_files_for_parcels(&server, &parcels).await?;


  for scene_file in scene_files {
    let path_str = "./assets/scenes/".to_string() + &scene_file.id.to_string();
    let scene_path = Path::new(&path_str);
    if !scene_path.exists() {
      let mut file_2dcl_exists = false;
      let mut file_json: Option<ContentFile> = None;

      for downloadable in scene_file.clone().content {
        if downloadable
            .filename
            .to_str()
            .unwrap()
            .ends_with("scene.2dcl")
        {
          fs::create_dir_all(format!("./assets/scenes/{}", scene_file.id))?;
          let filename = format!(
            "./assets/scenes/{}/{}",
            scene_file.id,
            downloadable.filename.to_str().unwrap()
          );
          
          println!("Downloading {}", filename);
          ContentClient::download(&server, downloadable.cid, &filename).await?;
      
          if let Some(scene) = read_scene_file(&filename) {
            let mut scene_path = PathBuf::from_str(&format!("./assets/scenes/{}", scene_file.id))?;
            scene_path.push(downloadable.filename.clone());
            parel_map.push((scene_path,scene.parcels));

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
          println!("finished download");

          break;
        }
      }
    }
  }

  Ok(parel_map)
}

fn make_road_scene_for_parcel(parcel: &Parcel) {
  
    let mut scene = dcl2d_ecs_v1::Scene::default();
    scene.parcels.push(parcel.clone());

    let mut background = dcl2d_ecs_v1::Entity::new("Background".to_string());
    let renderer = SpriteRenderer {
        sprite: "road-parcel.png".to_string(),
        layer: -1,
        ..default()
    };
    background.components.push(Box::new(renderer));

    let level = dcl2d_ecs_v1::Level {
        entities: vec![background],
        ..Default::default()
    };
    scene.levels.push(level);

    let mut buf: Vec<u8> = Vec::new();
    scene.serialize(&mut Serializer::new(&mut buf)).unwrap();
    let save_path =
        "./assets/scenes/road_".to_string() + &parcel.0.to_string() + "_" + &parcel.1.to_string();
    create_dir(&save_path).unwrap();
    let mut file = File::create(save_path.clone() + "/scene.2dcl").unwrap();
    file.write_all(&buf).unwrap();
    let save_path = save_path + "/assets";
    create_dir(&save_path).unwrap();
    fs::copy("./assets/road-parcel.png", save_path + "/road-parcel.png").unwrap();
}

pub fn read_scene(content: &[u8]) -> Option<dcl2d_ecs_v1::Scene> {
    let mut de = Deserializer::new(content);
    let scene: Result<dcl2d_ecs_v1::Scene, rmp_serde::decode::Error> =
        Deserialize::deserialize(&mut de);

    match scene {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn get_scene(roads_data: &mut RoadsData, parcel_map: &ParcelMap, parcel: &Parcel) -> (Result<dcl2d_ecs_v1::Scene, String>, PathBuf) {
    
  
  if is_road(parcel,roads_data)
  {
    let result = make_road_scene(roads_data, parcel);
    match result.0
    {
      Ok(scene) => return (Ok(scene),result.1),
      Err(_) =>{}
    }
  }

  let path = match get_parcel_file(parcel,parcel_map)
  {
    Some(v) => v,
    None => return (Err("Parcel not downloaded".to_string()), PathBuf::default()),
  };

  if path.exists() {
      if let Some(scene) = read_scene_file(&path) {
        let path = path.parent().unwrap().to_path_buf();
        let iter = path.iter().rev();
        let mut new_path = PathBuf::default();
        for i in iter {
            new_path.push(i);
        }
        new_path.pop();
        new_path.pop();
        let iter = new_path.iter().rev();

        let mut final_path = PathBuf::default();
        final_path.push("scenes");
        for i in iter {
            final_path.push(i);
        }
        return (Ok(scene), final_path);
      }
  }
  
  (Err("Parcel not downloaded".to_string()), PathBuf::default())
}

fn handle_tasks(
    mut commands: Commands,
    mut collision_map: ResMut<CollisionMap>,
    mut roads_data: ResMut<RoadsData>,
    mut parcel_map: ResMut<ParcelMap>,
    asset_server: Res<AssetServer>,
    mut tasks_downloading_scenes: Query<(Entity, &mut DownloadingScene)>,
    scenes_query: Query<(Entity, &crate::components::Scene)>,
) {

  for (entity, mut downloading_scene) in &mut tasks_downloading_scenes {
    if let Some(new_parcel_map_data) = future::block_on(future::poll_once(&mut downloading_scene.task)) {
      commands.entity(entity).despawn_recursive();

      if let Some(new_parcel_map_data) = new_parcel_map_data
      {
        for new_data in new_parcel_map_data
        {
          for parcel in new_data.1
          {
            add_parcel_file(parcel,new_data.0.clone(),&mut parcel_map);
          }
        }
        
        for parcel in &downloading_scene.parcels {
    
          let result = get_scene(
            &mut roads_data,
            &parcel_map,
            &parcel);

          if result.0.is_ok() {
            spawn_scene(
              &mut commands,
              &asset_server,
              result.0.unwrap(),
              result.1,
              &mut collision_map,
              SystemTime::now(),
              0,
            );
          }
        }  
      }
    }
  }

  for (entity_1, scene_1) in &scenes_query {
    for (entity_2, scene_2) in &scenes_query {
      if entity_1 != entity_2
          && (scene_1.name == "Sample Scene" || scene_2.name == "Sample Scene")
      {
        'outer: for parcel_1 in &scene_1.parcels {
          for parcel_2 in &scene_2.parcels {
            if *parcel_1 == *parcel_2 {
              if scene_1.name == "Sample Scene" {
                println!("Despawning empty_parcel {:?}", parcel_1);
                commands.entity(entity_1).despawn_recursive();
                break 'outer;
              }

              if scene_2.name == "Sample Scene" {
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
    collision_map: &mut CollisionMap,
) {
    let mut scene = dcl2d_ecs_v1::Scene::default();
    scene.parcels.push(parcel.clone());
    scene.name = "default_scene".to_string();
    let mut background = dcl2d_ecs_v1::Entity::new("Background".to_string());
    let renderer = SpriteRenderer {
        sprite: "default-parcel.png".to_string(),
        layer: -1,
        ..default()
    };
    background.components.push(Box::new(renderer));

    let level = dcl2d_ecs_v1::Level {
        entities: vec![background],
        ..Default::default()
    };
    scene.levels.push(level);

    spawn_scene(
        commands,
        asset_server,
        scene,
        "../",
        collision_map,
        SystemTime::now(),
        0,
    );
}

pub fn spawn_level<T>(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene: &dcl2d_ecs_v1::Scene,
    level_id: usize,
    path: T,
    collision_map: &mut CollisionMap,
    timestamp: SystemTime,
) -> Entity
where
    T: AsRef<Path>,
{
    let level_entity = commands.spawn().id();

    if scene.levels.len() <= level_id {
        return level_entity;
    }

    let level = &scene.levels[level_id];

    commands
        .entity(level_entity)
        .insert(Name::new(level.name.clone()))
        .insert(Visibility { is_visible: true })
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Transform::default())
        .insert(Level {
            name: level.name.clone(),
            timestamp,
            id: level_id,
        });

    for entity in level.entities.iter() {
        let spawned_entity = spawn_entity(
            commands,
            asset_server,
            path.as_ref(),
            collision_map,
            entity,
            scene,
        );
        commands.entity(level_entity).add_child(spawned_entity);
    }

    level_entity
}

pub fn spawn_scene<T>(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene: dcl2d_ecs_v1::Scene,
    path: T,
    collision_map: &mut CollisionMap,
    timestamp: SystemTime,
    level_id: usize,
) -> Entity
where
    T: AsRef<Path>,
{

    let scene_location: Vec3 = get_scene_center_location(&scene);
    let mut scene_data: Vec<u8> = Vec::new();
    scene
        .serialize(&mut Serializer::new(&mut scene_data))
        .unwrap();
    let scene_entity = commands
        .spawn()
        .insert(Visibility { is_visible: true })
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new(scene.name.clone()))
        .insert(Transform::from_translation(scene_location))
        .insert(crate::components::Scene {
            name: scene.name.clone(),
            parcels: scene.parcels.clone(),
            timestamp,
            scene_data,
            path: path.as_ref().to_path_buf(),
        })
        .id();

    if !scene.levels.is_empty() {
        let level_entity = spawn_level(
            commands,
            asset_server,
            &scene,
            level_id,
            path,
            collision_map,
            SystemTime::now(),
        );
        commands.entity(scene_entity).add_child(level_entity);
    }

    scene_entity
}

fn entity_anchor_to_anchor(anchor: dcl2d_ecs_v1::Anchor, size: Vec2) -> Anchor {
    match anchor {
        dcl2d_ecs_v1::Anchor::BottomCenter => Anchor::BottomCenter,
        dcl2d_ecs_v1::Anchor::BottomLeft => Anchor::BottomLeft,
        dcl2d_ecs_v1::Anchor::BottomRight => Anchor::BottomRight,
        dcl2d_ecs_v1::Anchor::Center => Anchor::Center,
        dcl2d_ecs_v1::Anchor::CenterLeft => Anchor::CenterLeft,
        dcl2d_ecs_v1::Anchor::CenterRight => Anchor::CenterRight,
        dcl2d_ecs_v1::Anchor::Custom(vec) => Anchor::Custom(
            Vec2::new(vec.x as f32 - size.x / 2.0, vec.y as f32 - size.y / 2.0) / size,
        ),
        dcl2d_ecs_v1::Anchor::TopCenter => Anchor::TopCenter,
        dcl2d_ecs_v1::Anchor::TopLeft => Anchor::TopLeft,
        dcl2d_ecs_v1::Anchor::TopRight => Anchor::TopRight,
    }
}

fn get_fixed_translation_by_anchor(
    size: &Vec2,
    translation: &Vec2,
    anchor: &dcl2d_ecs_v1::Anchor,
) -> Vec2 {
    match anchor {
        dcl2d_ecs_v1::Anchor::BottomCenter => Vec2 {
            x: translation.x - size.x / 2.0,
            y: translation.y + size.y,
        },
        dcl2d_ecs_v1::Anchor::BottomLeft => Vec2 {
            x: translation.x,
            y: translation.y + size.y,
        },
        dcl2d_ecs_v1::Anchor::BottomRight => Vec2 {
            x: translation.x - size.x,
            y: translation.y + size.y,
        },
        dcl2d_ecs_v1::Anchor::Center => Vec2 {
            x: translation.x - size.x / 2.0,
            y: translation.y + size.y / 2.0,
        },
        dcl2d_ecs_v1::Anchor::CenterLeft => Vec2 {
            x: translation.x,
            y: translation.y + size.y / 2.0,
        },
        dcl2d_ecs_v1::Anchor::CenterRight => Vec2 {
            x: translation.x - size.x,
            y: translation.y + size.y / 2.0,
        },
        dcl2d_ecs_v1::Anchor::Custom(vec) => Vec2 {
            x: translation.x - vec.x as f32,
            y: translation.y + size.y - vec.y as f32,
        },
        dcl2d_ecs_v1::Anchor::TopCenter => Vec2 {
            x: translation.x - size.x / 2.0,
            y: translation.y,
        },
        dcl2d_ecs_v1::Anchor::TopLeft => *translation,
        dcl2d_ecs_v1::Anchor::TopRight => Vec2 {
            x: translation.x - size.x,
            y: translation.y,
        },
    }
}

fn spawn_entity<T>(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: T,
    collision_map: &mut CollisionMap,
    entity: &dcl2d_ecs_v1::Entity,
    scene: &dcl2d_ecs_v1::Scene,
) -> Entity
where
    T: AsRef<Path>,
{
    let mut transform = Transform::identity();
    for component in entity.components.iter() {
      
        if let Some(source_transform) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::Transform>()
        {
            let location = Vec2::new(
                source_transform.location.x as f32,
                source_transform.location.y as f32,
            );
            let scale = Vec2::new(source_transform.scale.x, source_transform.scale.y);

            transform.translation += location.extend(location.y * -1.0);
            transform.rotation = Quat::from_euler(
                EulerRot::XYZ,
                source_transform.rotation.x.to_radians(),
                source_transform.rotation.y.to_radians(),
                source_transform.rotation.z.to_radians(),
            );

            transform.scale = scale.extend(1.0);
        };
    }

    //Spawning Entity
    let spawned_entity = commands
        .spawn()
        .insert(Name::new(entity.name.clone()))
        .insert(Visibility { is_visible: true })
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(transform)
        .id();

    // Inserting components
    for component in entity.components.iter() {
        if let Some(sprite_renderer) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::SpriteRenderer>()
        {
            transform.translation = Vec3 {
                x: transform.translation.x,
                y: transform.translation.y,
                z: transform.translation.z + sprite_renderer.layer as f32 * 500.0,
            };

            commands.entity(spawned_entity).insert(transform);

            let mut image_path = path.as_ref().to_path_buf();
            image_path.push("assets");
            image_path.push(&sprite_renderer.sprite);
            let texture: Handle<Image> = asset_server.load(image_path);
            commands.entity(spawned_entity).insert(texture);
            let mut sprite_path = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
            sprite_path.push("assets");
            sprite_path.push(path.as_ref());
            sprite_path.push("assets");
            sprite_path.push(&sprite_renderer.sprite);
            let image_size = match size(sprite_path) {
                Ok(v) => Vec2::new(v.width as f32, v.height as f32),
                Err(_) => Vec2::new(0.0, 0.0),
            };
            
            let sprite = Sprite {
                color: Color::Rgba {
                    red: (&sprite_renderer).color.r,
                    green: (&sprite_renderer).color.g,
                    blue:(&sprite_renderer).color.b,
                    alpha: (&sprite_renderer).color.a,
                },
                anchor: entity_anchor_to_anchor((&sprite_renderer).anchor.clone(), image_size),
                flip_x: (&sprite_renderer).flip.x,
                flip_y: (&sprite_renderer).flip.y,
                ..default()
            };
            commands.entity(spawned_entity).insert(sprite);
        }

        if let Some(collider) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::BoxCollider>()
        {
            let box_collider = BoxCollider {
                center: Vec2::new(collider.center.x as f32, collider.center.y as f32),
                size: Vec2::new(collider.size.width as f32, collider.size.height as f32),
                collision_type: collider.collision_type.clone(),
            };
            commands.entity(spawned_entity).insert(box_collider);
        }

        if let Some(collider) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::CircleCollider>()
        {
            let circle_collider = CircleCollider {
                center: Vec2::new(collider.center.x as f32, collider.center.y as f32),
                radius: collider.radius,
            };
            commands.entity(spawned_entity).insert(circle_collider);
        }

        if let Some(collider) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::CircleCollider>()
        {
            let circle_collider = CircleCollider {
                center: Vec2::new(collider.center.x as f32, collider.center.y as f32),
                radius: collider.radius,
            };
            commands.entity(spawned_entity).insert(circle_collider);
        }

        if let Some(collider) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::MaskCollider>()
        {
            let mut sprite_path = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
            sprite_path.push("assets");
            sprite_path.push(path.as_ref());
            sprite_path.push("assets");
            sprite_path.push(&collider.sprite);

            if let Ok(mut reader) = ImageReader::open(sprite_path) {
                reader.set_format(ImageFormat::Png);
                if let Ok(DynamicImage::ImageRgba8(image)) = reader.decode() {
                    let mut pixels = image.pixels();
                    let rows = image.rows().len();
                    let columns = pixels.len() / rows;
                    let world_transform = transform.translation + get_scene_center_location(scene);

                    let fixed_translation = get_fixed_translation_by_anchor(
                        &Vec2 {
                            x: columns as f32,
                            y: rows as f32,
                        },
                        &world_transform.truncate(),
                        &collider.anchor,
                    );

                    let mut index = 0;
                    let channel = collider.channel.clone() as usize;

                    while pixels.len() > 0 {
                        if pixels.next().unwrap()[channel] > 0 {
                            let tile_location = fixed_translation
                                + (Vec2::new(
                                    (index % columns) as f32,
                                    (index / columns) as f32 * -1.0,
                                ) * super::collision::TILE_SIZE);
                            let collision_tile = CollisionTile {
                                location: tile_location,
                                colliision_type: collider.collision_type.clone(),
                                entity: Some(spawned_entity),
                            };
                            collision_map.tiles.push(collision_tile);
                        }
                        index += 1;
                    }
                }
            }
        }

        if let Some(level_change) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::triggers::LevelChange>(
        ) {
            let mut new_level_id = 0;

            for i in 0..scene.levels.len() {
                if scene.levels[i].name == level_change.level {
                    new_level_id = i;
                    break;
                }
            }

            let scene_center_location = get_scene_center_location(scene);
            let level_change_component = LevelChange {
                level: new_level_id,
                spawn_point: Vec2::new(
                    level_change.spawn_point.x as f32 + scene_center_location.x,
                    level_change.spawn_point.y as f32 + scene_center_location.y,
                ),
            };
            commands
                .entity(spawned_entity)
                .insert(level_change_component);
        }
    }

    for child_entity in entity.children.iter() {
      
        let spawned_child_entity = spawn_entity(
            commands,
            asset_server,
            path.as_ref(),
            collision_map,
            child_entity,
            scene,
        );
        commands
            .entity(spawned_entity)
            .add_child(spawned_child_entity);
    }
    spawned_entity
}



