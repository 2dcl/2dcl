use std::{path::PathBuf, fs::File, io::BufReader};
use crate::renderer::scene_loader::JsonScene;
use serde::Serialize;
use rmp_serde::*;
use std::io::prelude::*;

pub fn run(json_path: PathBuf, mut build_path: PathBuf) {

   
    if let Ok(file) = File::open(json_path)
    {
        let reader = BufReader::new(file);
        let scene: serde_json::Result<JsonScene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();
            let mut buf: Vec<u8> = Vec::new();
            scene.serialize(&mut Serializer::new(&mut buf)).unwrap();
            build_path.set_extension("2dcl");
            let mut file = File::create(build_path).unwrap();
            file.write_all(&buf);
        }
    }
}



