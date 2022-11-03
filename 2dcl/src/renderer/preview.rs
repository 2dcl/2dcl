use std::{path::PathBuf, str::FromStr};

use bevy::prelude::*;
use dcl_common::Parcel;

use futures_lite::future;
use super::{scene_loader::{read_scene_file, spawn_scene, TextureLoading, SpriteLoading, AlphaColliderLoading}, collision::CollisionMap}; 



pub struct PreviewPath
{
    pub path: PathBuf
}

pub struct PreviewPlugin;

impl Plugin for PreviewPlugin{
    fn build(&self, app: &mut App)
    {
        app
        .add_system(handle_tasks)
        .add_startup_system(setup);
    }
}

fn setup(mut commands:  Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    preview_path: Res<PreviewPath>,
)
{
 
    load_preview_scene(&mut commands, &asset_server, &mut texture_atlases, &preview_path.path);
}


pub fn load_preview_scene(    
    mut commands: &mut Commands,
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
        scene.path = Some(path.parent().unwrap().to_path_buf());
        let scene_entity = spawn_scene(commands,asset_server,texture_atlases,scene);
    }
   
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