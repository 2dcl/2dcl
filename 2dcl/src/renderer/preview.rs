
use crate::renderer::scene_loader::SceneComponent;
use bevy::prelude::*;
use super::scene_loader; 
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
use std::time::SystemTime;

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

pub struct PreviewPlugin;

pub struct SceneHandler(Handle<SceneAsset>);

impl Plugin for PreviewPlugin{
    fn build(&self, app: &mut App)
    {
        app
        .add_system(scene_reload)
        .add_startup_system(setup);
    }
}

fn setup(mut commands:  Commands,
    asset_server: Res<AssetServer>,
)
{
    asset_server.watch_for_changes().unwrap();

    let handler : Handle<SceneAsset> = asset_server.load("../scene.2dcl");

    commands.insert_resource(SceneHandler(handler));
}


fn scene_reload(
    mut commands: Commands,
    scene_assets: ResMut<Assets<SceneAsset>>,
    asset_server: Res<AssetServer>,
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
                    scene_loader::spawn_scene(&mut commands, &asset_server, scene, "../",timestamp);
                }

            }
            else 
            {
                let timestamp = scene.timestamp;
                let scene = scene_loader::read_scene(&scene.bytes).unwrap();
                scene_loader::spawn_scene(&mut commands, &asset_server, scene, "../",timestamp);
            }
        }
        None => {},
    }
}

