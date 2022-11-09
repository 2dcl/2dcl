use std::io::Write;
use std::fs::File;

use dcl2d_ecs_v1::Anchor;
use serde::ser::Serialize;
use rmp_serde::Serializer;

fn main()
{
  let json = include_str!("../fixtures/anchor/custom.json");
  let element : Anchor = serde_json::from_str(json.as_ref()).unwrap();
  let mut result: Vec<u8> = Vec::new();
  element.serialize(&mut Serializer::new(&mut result)).unwrap();
  let mut file = File::create("custom.mp").unwrap();
  file.write_all(&result).unwrap();
}
