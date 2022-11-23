use dcl2d_ecs_v1::components::MaskCollider;
use std::fs::File;
use std::io::Write;

use rmp_serde::Serializer;
use serde::ser::Serialize;

fn main() {
    let json = include_str!("../fixtures/components/mask_collider.json");
    let element: MaskCollider = serde_json::from_str(json).unwrap();
    let mut result: Vec<u8> = Vec::new();
    element
        .serialize(&mut Serializer::new(&mut result))
        .unwrap();
    let mut file = File::create("mask_collider.mp").unwrap();
    file.write_all(&result).unwrap();
}
