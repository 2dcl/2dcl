use bevy::prelude::*;
use dcl_common::Parcel;
use glob::glob;
use rmp_serde::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use super::dcl_3d_scene;
use super::error::ScenesIOError;
use super::scene_maker::{is_road, make_road_scene, RoadsData};

#[derive(Debug, Clone, Default)]
pub struct SceneFilesMap {
    pub map: HashMap<(i16, i16), SceneFileData>,
}

#[derive(Debug, Clone, Default)]
pub struct SceneFileData {
    pub path: PathBuf,
    pub parcels: Vec<Parcel>,
}

#[derive(Debug, Default)]
pub struct SceneData {
    pub scene: dcl2d_ecs_v1::Scene,
    pub parcels: Vec<Parcel>,
    pub path: PathBuf,
}

pub struct ScenesIOPlugin;

impl Plugin for ScenesIOPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    match get_scene_files_map() {
        Ok(scene_files_map) => commands.insert_resource(scene_files_map),
        Err(e) => println!("error:{}", e),
    }
}

fn get_scene_files_map() -> dcl_common::Result<SceneFilesMap> {
    let mut scene_files_map = SceneFilesMap::default();

    let paths = std::fs::read_dir("./assets/scenes").unwrap();

    for path in paths.flatten() {
        match refresh_path(path.path(), &mut scene_files_map) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        };
    }

    Ok(scene_files_map)
}

pub fn read_scene_file<P>(file_path: P) -> Option<dcl2d_ecs_v1::Scene>
where
    P: AsRef<Path>,
{
    if let Ok(file) = File::open(&file_path) {
        let reader = BufReader::new(file);
        let mut de = Deserializer::new(reader);
        let scene: Result<dcl2d_ecs_v1::Scene, rmp_serde::decode::Error> =
            Deserialize::deserialize(&mut de);

        match scene {
            Ok(v) => return Some(v),
            Err(e) => {
                println!("error at scene des {:?}", e);
                return None;
            }
        }
    } else {
        println!("no path: {:?}", file_path.as_ref());
    }

    None
}

pub fn get_parcel_file_data(
    parcel: &Parcel,
    scene_files_map: &SceneFilesMap,
) -> Option<SceneFileData> {
    scene_files_map.map.get(&(parcel.0, parcel.1)).cloned()
}

pub fn get_scene(
    roads_data: &mut RoadsData,
    scene_files_map: &SceneFilesMap,
    parcel: &Parcel,
) -> Option<SceneData> {
    if is_road(parcel, roads_data) {
        if let Ok(scene) = make_road_scene(roads_data, parcel) {
            return Some(scene);
        }
    }

    let scene_file_data = match get_parcel_file_data(parcel, scene_files_map) {
        Some(v) => v,
        None => return None,
    };

    if scene_file_data.path.exists() {
        if let Some(scene) = read_scene_file(&scene_file_data.path) {
            let mut path: PathBuf = scene_file_data.path.iter().rev().collect();
            path.pop();
            let mut path: PathBuf = path.iter().rev().collect();
            path.pop();

            let scene_data = SceneData {
                scene,
                parcels: scene_file_data.parcels,
                path,
            };
            return Some(scene_data);
        }
    }

    None
}

pub fn read_scene_u8(content: &[u8]) -> Option<dcl2d_ecs_v1::Scene> {
    let mut de = Deserializer::new(content);
    let scene: Result<dcl2d_ecs_v1::Scene, rmp_serde::decode::Error> =
        Deserialize::deserialize(&mut de);

    match scene {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

pub fn refresh_path(path: PathBuf, scene_files_map: &mut SceneFilesMap) -> dcl_common::Result<()> {
    let pattern_2dcl = match path.to_str() {
        Some(v) => format!("{}/**/scene.2dcl", v,),
        None => {
            return Err(Box::new(ScenesIOError::InvalidPath(path)));
        }
    };

    let json_path = match path.to_str() {
        Some(v) => format!("{}/scene.json", v,),
        None => {
            return Err(Box::new(ScenesIOError::InvalidPath(path)));
        }
    };

    if let Ok(scene_3dcl) = read_3dcl_scene(json_path) {
        let parcels = scene_3dcl.scene.parcels.clone();
        if let Some(entry) = glob(pattern_2dcl.as_str())
            .expect("Failed to read glob pattern")
            .next()
        {
            match entry {
                Ok(path) => {
                    let scene_file_data = SceneFileData { path, parcels };

                    for parcel in scene_3dcl.scene.parcels {
                        scene_files_map
                            .map
                            .insert((parcel.0, parcel.1), scene_file_data.clone());
                    }
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
    }

    Ok(())
}

pub fn read_3dcl_scene<P>(filename: P) -> dcl_common::Result<dcl_3d_scene::DCL3dScene>
where
    P: AsRef<Path>,
{
    if let Ok(file) = File::open(&filename) {
        let reader = BufReader::new(file);
        let result: serde_json::Result<dcl_3d_scene::DCL3dScene> = serde_json::from_reader(reader);
        match result {
            Ok(scene) => return Ok(scene),
            Err(e) => return Err(Box::new(e)),
        }
    }

    return Err(Box::new(ScenesIOError::InvalidPath(
        filename.as_ref().to_path_buf(),
    )));
}
