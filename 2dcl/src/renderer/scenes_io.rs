use bevy::prelude::*;
use dcl_common::Parcel;
use glob::glob;
use rmp_serde::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct ParcelMap {
    pub map: HashMap<(i16, i16), PathBuf>,
}

pub struct ScenesIOPlugin;

impl Plugin for ScenesIOPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    match get_parcel_map() {
        Ok(parcel_map) => commands.insert_resource(parcel_map),
        Err(e) => println!("error:{}", e),
    }
}

fn get_parcel_map() -> dcl_common::Result<ParcelMap> {
    let mut parcel_map = ParcelMap::default();

    for entry in glob("./assets/scenes/**/scene.2dcl").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Some(scene) = read_scene_file(&path) {
                    for parcel in scene.parcels {
                        parcel_map.map.insert((parcel.0, parcel.1), path.clone());
                    }
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(parcel_map)
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
            Err(_) => return None,
        }
    } else {
        println!("no path: {:?}", file_path.as_ref());
    }

    None
}

pub fn get_parcel_file(parcel: &Parcel, parcel_map: &ParcelMap) -> Option<PathBuf> {
    match parcel_map.map.get(&(parcel.0, parcel.1)) {
        Some(v) => Some(v.clone()),
        None => None,
    }
}

pub fn add_parcel_file(parcel: Parcel, path: PathBuf, parcel_map: &mut ParcelMap) {
    println!("adding parcel file: {:?} > {:?}", parcel, path);
    parcel_map.map.insert((parcel.0, parcel.1), path);
}
