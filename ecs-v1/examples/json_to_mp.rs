use dcl2d_ecs_v1::Scene;

use std::fs::File;
use std::io::Write;

use rmp_serde::Serializer;
use serde::ser::Serialize;

fn main() {
    let json = include_str!("../fixtures/scene.json");
    let element: Scene = serde_json::from_str(json).unwrap();
    let mut result: Vec<u8> = Vec::new();
    element
        .serialize(&mut Serializer::new(&mut result))
        .unwrap();
    let mut file = File::create("scene.mp").unwrap();
    file.write_all(&result).unwrap();
}
