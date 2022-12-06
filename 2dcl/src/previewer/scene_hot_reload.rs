use crate::renderer::scenes_io::read_scene_u8;
use crate::renderer::scenes_io::SceneData;
use crate::renderer::CollisionMap;
use crate::renderer::PlayerComponent;
use bevy::asset::Handle;
use bevy::prelude::*;
use dcl_common::Parcel;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};

use crate::components::Level;
use crate::components::Scene;
use crate::renderer::scene_loader;

use rmp_serde::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "1b06c21a-5ecd-11ed-9b6a-0242ac120002"]
pub struct SceneAsset {
    bytes: Vec<u8>,
    timestamp: SystemTime,
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
            load_context.set_default_asset(LoadedAsset::new(SceneAsset {
                bytes: bytes.to_vec(),
                timestamp: SystemTime::now(),
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["2dcl"]
    }
}

pub struct SceneHotReloadPlugin;

pub struct SceneHandler(Handle<SceneAsset>);

impl Plugin for SceneHotReloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(scene_reload)
            .add_system(level_change)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let result = asset_server.watch_for_changes();

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return;
    }

    let handler: Handle<SceneAsset> = asset_server.load("../scene.2dcl");

    commands.insert_resource(SceneHandler(handler));
}

fn scene_reload(
    mut commands: Commands,
    scene_assets: ResMut<Assets<SceneAsset>>,
    asset_server: Res<AssetServer>,
    scene_handlers: Res<SceneHandler>,
    mut scenes: Query<(Entity, &mut Scene)>,
    mut collision_map: ResMut<CollisionMap>,
    player: Query<&PlayerComponent>,
) {
    let level_id = match player.get_single() {
        Ok(player) => player.current_level,
        _ => 0,
    };

    if let Some(scene) = scene_assets.get(&scene_handlers.0) {
        if let Ok((entity, current_scene)) = scenes.get_single_mut() {
            if scene.timestamp != current_scene.timestamp {
                commands.entity(entity).despawn_recursive();
                let timestamp = scene.timestamp;
                if let Some(scene) = read_scene_u8(&scene.bytes) {
                    let scene_data = SceneData {
                        scene,
                        parcels: vec![Parcel(0, 0)],
                        path: PathBuf::from_str("../").unwrap(),
                    };

                    scene_loader::spawn_scene(
                        &mut commands,
                        &asset_server,
                        &scene_data,
                        &mut collision_map,
                        timestamp,
                        level_id,
                    );
                }
            }
        } else {
            let timestamp = scene.timestamp;
            if let Some(scene) = read_scene_u8(&scene.bytes) {
                let scene_data = SceneData {
                    scene,
                    parcels: vec![Parcel(0, 0)],
                    path: PathBuf::from_str("../").unwrap(),
                };

                scene_loader::spawn_scene(
                    &mut commands,
                    &asset_server,
                    &scene_data,
                    &mut collision_map,
                    timestamp,
                    level_id,
                );
            }
        }
    }
}

pub fn level_change(
    mut player_query: Query<&PlayerComponent>,
    scene_query: Query<(Entity, &Scene)>,
    level_query: Query<(Entity, &Level)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_map: ResMut<CollisionMap>,
) {
    //Find the player
    let player = player_query.get_single_mut();

    if player.is_err() {
        return;
    }

    let player = player.unwrap();

    let scene_query = scene_query.get_single();
    if scene_query.is_err() {
        return;
    }

    let (scene_entity, scene) = scene_query.unwrap();
    let current_level = player.current_level;

    let mut should_spawn = true;

    //We check if we're on the correct level
    for (level_entity, level) in level_query.iter() {
        if current_level != level.id {
            //Despawn level for current parcel
            commands.entity(level_entity).despawn_recursive();

            //Clear collision map
            collision_map.tiles.clear();
        } else {
            should_spawn = false;
        }
    }

    if should_spawn {
        let mut de = Deserializer::from_read_ref(&scene.serialized_data);
        let deserialized_scene: dcl2d_ecs_v1::Scene = Deserialize::deserialize(&mut de).unwrap();
        let scene_data = SceneData {
            scene: deserialized_scene,
            parcels: vec![Parcel(0, 0)],
            path: scene.path.clone(),
        };

        scene_loader::spawn_level(
            &mut commands,
            &asset_server,
            &scene_data,
            current_level,
            &mut collision_map,
            SystemTime::now(),
        );
        let level_entity = scene_loader::spawn_level(
            &mut commands,
            &asset_server,
            &scene_data,
            current_level,
            &mut collision_map,
            SystemTime::now(),
        );
        commands.entity(scene_entity).add_child(level_entity);
    }
}
