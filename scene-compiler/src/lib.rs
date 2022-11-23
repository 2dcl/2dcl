mod error;

use crate::error::SceneCompileError;
use dcl2d_ecs_v1::Scene;
use dcl_common::Result;
use fs_extra::dir::CopyOptions;
use rmp_serde::*;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub fn compile<T, U>(source_path: T, destination_path: U) -> Result<()>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    let mut assets_source_path = source_path.as_ref().to_path_buf();
    assets_source_path.push("assets");
    let mut source_path = source_path.as_ref().to_path_buf();

    let assets_destination_path = destination_path.as_ref().to_path_buf();
    let mut destination_path = destination_path.as_ref().to_path_buf();

    if !source_path.exists() || !source_path.is_dir() {
        return Err(Box::new(SceneCompileError::SourceNotDirectory));
    }

    source_path.push("scene.json");

    let file = File::open(source_path.clone())?;
    let reader = BufReader::new(file);
    let scene: Scene = serde_json::from_reader(reader)?;

    if scene.parcels.is_empty() {
        return Err(Box::new(SceneCompileError::NoParcels));
    }

    let mut buf: Vec<u8> = Vec::new();
    scene.serialize(&mut Serializer::new(&mut buf))?;

    //Todo check componets

    if !destination_path.exists() {
        fs::create_dir(&destination_path)?;
    } else if !destination_path.is_dir() {
        return Err(Box::new(SceneCompileError::DestinationNotDirectory));
    }

    destination_path.push("scene.2dcl");
    let mut file = File::create(&destination_path)?;

    file.write_all(&buf)?;

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    fs_extra::dir::copy(assets_source_path, assets_destination_path, &options)?;
    Ok(())
}
