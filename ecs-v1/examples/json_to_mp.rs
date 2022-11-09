use dcl2d_ecs_v1::components::MaskCollider;
use std::io::Write;
use std::fs::File;


use serde::ser::Serialize;
use rmp_serde::Serializer;

fn main()
{
  let json = include_str!("../fixtures/components/mask_collider.json");
  let element : MaskCollider = serde_json::from_str(json.as_ref()).unwrap();
  let mut result: Vec<u8> = Vec::new();
  element.serialize(&mut Serializer::new(&mut result)).unwrap();
  let mut file = File::create("mask_collider.mp").unwrap();
  file.write_all(&result).unwrap();
}
