
use crate::renderer::scene_loader::SceneComponent;
use std::{path::PathBuf};

use bevy::{prelude::*, time::TimeSender};


use dcl_common::Parcel;

use futures_lite::future;


use super::{collision::CollisionMap, scene_loader::{self, AlphaColliderLoading, SpriteLoading, TextureLoading, read_scene_file, spawn_scene}}; 


use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
//     prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "1b06c21a-5ecd-11ed-9b6a-0242ac120002"]
pub struct SceneAsset {
    bytes: Vec<u8>,
    timestamp: SystemTime
}

#[derive(Default)]
pub struct SceneAssetLoader;

impl AssetLoader for SceneAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            load_context.set_default_asset(LoadedAsset::new(SceneAsset{ 
                bytes: bytes.to_vec(),
                timestamp: SystemTime::now()
            }));
            println!("Loaded Scene Asset");
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["2dcl"]
    }
}

pub struct PreviewPath
{
    pub path: PathBuf
}

pub struct PreviewPlugin;

pub struct SceneHandler(Handle<SceneAsset>);

impl Plugin for PreviewPlugin{
    fn build(&self, app: &mut App)
    {
        app
        .add_system(handle_tasks)
        .add_system(scene_reload)
        .add_startup_system(setup);
    }
}

fn setup(mut commands:  Commands,
    asset_server: Res<AssetServer>,
    scene_assets: Res<Assets<SceneAsset>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{
    asset_server.watch_for_changes().unwrap();

    let handler : Handle<SceneAsset> = asset_server.load("../scene.2dcl");

    commands.insert_resource(SceneHandler(handler));
}



pub fn load_preview_scene(    
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    path: &PathBuf
)
{
    let scene = read_scene_file(path);
    if scene.is_some() 
    {
        let mut scene = scene.unwrap();
        scene.parcels = vec![Parcel(0,0)];
        // scene.path = Some(path.parent().unwrap().to_path_buf());
        // let scene_entity = spawn_scene(commands,asset_server,texture_atlases,scene);
    }
   
}

fn scene_reload(
    mut commands: Commands,
    mut scene_assets: ResMut<Assets<SceneAsset>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    scene_handlers: Res<SceneHandler>,
    mut scenes: Query<(Entity, &mut SceneComponent)>
)
{
    let handler = scene_assets.get(&scene_handlers.0);

    match handler {
        Some(scene) => {
            if scenes.iter().len() > 0 {
                let (entity, current_scene)  = scenes.single_mut();

                if scene.timestamp != current_scene.timestamp {
                    commands.entity(entity).despawn_recursive();
                    let timestamp = scene.timestamp;
                    let scene = scene_loader::read_scene(&scene.bytes).unwrap();
                    scene_loader::spawn_scene(&mut commands, &asset_server, &mut texture_atlases, scene, timestamp);
                }

            }
            else 
            {
                let timestamp = scene.timestamp;
                let scene = scene_loader::read_scene(&scene.bytes).unwrap();
                scene_loader::spawn_scene(&mut commands, &asset_server, &mut texture_atlases, scene, timestamp);
            }
        }
        None => {},
    }

    



    // for (_entity, mut scene_component) in &mut scenes {
    //     println!("Scene: {} {:?}", scene_component.name, scene_component.timestamp);
    // }


    // if let Some(scene_asset) = future::block_on(future::poll_once(&mut task.0)) {
    //     let scene_asset = scene_assets.get(&scene_asset).unwrap();
    //     let scene = scene_loader::read_scene(&scene_asset.bytes).unwrap();
    //     scene_loader::spawn_scene(&mut commands, &asset_server, &mut texture_atlases, scene);
    // }
}

fn handle_tasks(
    mut commands: Commands,
    mut collision_map: ResMut<CollisionMap>,
    mut tasks_texture_loading: Query<(Entity, &mut TextureLoading)>,
    mut tasks_sprite_loading: Query<(Entity, &mut SpriteLoading)>,
    mut tasks_alpha_collider_loading: Query<(Entity, &mut AlphaColliderLoading)>,
) 
{ 
    for (entity, mut task) in &mut tasks_texture_loading {
        if let Some(image) = future::block_on(future::poll_once(&mut task.0)) {

            commands.entity(entity).insert(image);
            commands.entity(entity).remove::<TextureLoading>();
        }
    }

    for (entity, mut task) in &mut tasks_sprite_loading {
        if let Some(sprite) = future::block_on(future::poll_once(&mut task.0)) {
    
            commands.entity(entity).insert(sprite);
            commands.entity(entity).remove::<SpriteLoading>();
        }
    }

    for (entity, mut task) in &mut tasks_alpha_collider_loading {
        if let Some(collision) = future::block_on(future::poll_once(&mut task.0)) {
            let mut collision = collision.clone();
            collision_map.collision_locations.append(&mut collision);
            commands.entity(entity).remove::<AlphaColliderLoading>();
        }
    }
}
