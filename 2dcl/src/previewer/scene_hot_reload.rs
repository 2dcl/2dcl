use crate::bundles::get_scene_center_location;
use crate::components;
use crate::renderer::scene_loader::loading_sprites_tasks_handler;
use crate::renderer::scene_loader::DespawnedEntities;
use crate::renderer::scenes_io::read_scene_u8;
use crate::renderer::scenes_io::SceneData;
use crate::resources;
use bevy::asset::Handle;
use bevy::prelude::*;
use dcl_common::Parcel;
use notify::Event;
use notify::EventKind::Modify;
use notify::RecursiveMode;
use notify::Watcher;
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
    pub bytes: Vec<u8>,
    pub timestamp: SystemTime,
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
#[derive(Default, Resource)]
pub struct SceneHandler(pub Handle<SceneAsset>);

impl Plugin for SceneHotReloadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DespawnedEntities::default())
            .add_system(scene_reload)
            .add_system(level_change)
            .add_system(
                loading_sprites_tasks_handler
                    .after(level_change)
                    .after(scene_reload),
            )
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    /*  let result = asset_server.watch_for_changes();

        if result.is_err() {
            println!("{}", result.unwrap_err());
            return;
        }
    */
    let handler: Handle<SceneAsset> = asset_server.load("../scene.2dcl");
    commands.insert_resource(SceneHandler(handler));

    let mut watch_path = std::env::current_dir().unwrap_or_default();
    watch_path.pop();

    println!("watching {:?}", watch_path);
    let asset_server = asset_server.clone();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            let Event {
                kind,
                paths,
                attrs: _,
            } = event;

            println!("event triggered:{:?},{:?}", kind, paths);
            if let Modify(_) = kind {
                for path in paths {
                    if path.ends_with("scene.json") {
                        if let Err(error) = scene_compiler::compile(&path, "./build") {
                            println!("Error compiling: {}", error)
                        } else {
                            asset_server.reload_asset("../scene.2dcl");
                        }
                    }
                }
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();

    watcher
        .watch(&watch_path, RecursiveMode::Recursive)
        .unwrap();
}

fn scene_reload(
    mut commands: Commands,
    scene_assets: ResMut<Assets<SceneAsset>>,
    asset_server: Res<AssetServer>,
    scene_handlers: Res<SceneHandler>,
    mut scenes: Query<(Entity, &Scene)>,
    mut collision_map: ResMut<resources::CollisionMap>,
    mut player_query: Query<(&components::Player, &mut Transform)>,
    mut despawned_entities: ResMut<DespawnedEntities>,
) {
    if let Ok((player, mut player_transform)) = player_query.get_single_mut() {
        if let Some(scene) = scene_assets.get(&scene_handlers.0) {
            if let Ok((entity, current_scene)) = scenes.get_single_mut() {
                if scene.timestamp != current_scene.timestamp {
                    despawned_entities.entities.push(entity);
                    commands.entity(entity).despawn_recursive();
                    let timestamp = scene.timestamp;
                    if let Some(mut scene) = read_scene_u8(&scene.bytes) {
                        scene.timestamp = timestamp;
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
                            player.current_level,
                        );

                        let scene_center = get_scene_center_location(&scene_data);
                        player_transform.translation =
                            match player.current_level < scene_data.scene.levels.len() {
                                true => {
                                    let spawn_point = scene_data.scene.levels[player.current_level]
                                        .spawn_point
                                        .clone();
                                    Vec3 {
                                        x: spawn_point.x as f32 + scene_center.x,
                                        y: spawn_point.y as f32 + scene_center.y,
                                        z: (spawn_point.y as f32 + scene_center.y) * -1.0,
                                    }
                                }
                                false => scene_center,
                            }
                    }
                }
            } else {
                let timestamp = scene.timestamp;
                if let Some(mut scene) = read_scene_u8(&scene.bytes) {
                    scene.timestamp = timestamp;
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
                        player.current_level,
                    );

                    let scene_center = get_scene_center_location(&scene_data);
                    player_transform.translation =
                        match player.current_level < scene_data.scene.levels.len() {
                            true => {
                                let spawn_point = scene_data.scene.levels[player.current_level]
                                    .spawn_point
                                    .clone();
                                Vec3 {
                                    x: spawn_point.x as f32 + scene_center.x,
                                    y: spawn_point.y as f32 + scene_center.y,
                                    z: (spawn_point.y as f32 + scene_center.y) * -1.0,
                                }
                            }
                            false => scene_center,
                        }
                }
            }
        }
    }
}

pub fn level_change(
    player_query: Query<&components::Player>,
    scene_query: Query<(Entity, &Scene)>,
    level_query: Query<&Level>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_map: ResMut<resources::CollisionMap>,
    mut despawned_entities: ResMut<DespawnedEntities>,
) {
    //Find the player
    let player = player_query.get_single();

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
    for level in level_query.iter() {
        if current_level != level.id {
            //Despawn level for current parcel
            despawned_entities.entities.push(scene_entity);
            commands.entity(scene_entity).despawn_recursive();
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

        scene_loader::spawn_scene(
            &mut commands,
            &asset_server,
            &scene_data,
            &mut collision_map,
            current_level,
        );
    }
}
