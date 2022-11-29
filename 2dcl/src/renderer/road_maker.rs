
use dcl_common::{Result, Parcel};
use rmp_serde::encode::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write, BufReader};
use bevy::prelude::*;
use rmp_serde::*;
use super::scene_loader::parcel_to_world_location;
//pub fn spawn_road(location: Vec3, )

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RoadsData
{
  parcels:Vec<Parcel>
}


pub struct RoadMakerPlugin;

impl Plugin for RoadMakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(
  mut commands: Commands,
)
{
  println!("setup");
  match read_roads_data()
  {
    Ok(roads_data) => commands.insert_resource(roads_data),
    Err(e) => println!("error:{}",e)
  }


}

pub fn spawn_road_for_parcel(parcel: &Parcel)
{
  let location = parcel_to_world_location(&parcel);
  
}

pub fn make_road_scene(roads_data: &RoadsData, parcel: &Parcel) -> Result<dcl2d_ecs_v1::Scene>
{

  let mut entities: Vec<dcl2d_ecs_v1::Entity> = Vec::new();

  entities.push(make_background_entity());

  let level = dcl2d_ecs_v1::Level
  {
    name:format!("Road {} - {}", &parcel.0, &parcel.1),
    entities,
    ..default()
  };
  let scene = dcl2d_ecs_v1::Scene
  {
    id:0,
    name:format!("Road {} - {}", &parcel.0, &parcel.1),
    levels: vec![level],
    parcels: vec![parcel.clone()]
  };

  Ok(scene)
}


fn make_background_entity() -> dcl2d_ecs_v1::Entity
{
  let renderer = dcl2d_ecs_v1::components::SpriteRenderer
  {
    sprite:"road-background".to_string(),
    layer:-3,
    anchor: dcl2d_ecs_v1::Anchor::Center,
    ..default()
  };

  let transform = dcl2d_ecs_v1::components::Transform
  {
    location: dcl2d_ecs_v1::Vec2{x:0,y:0},
    rotation: dcl2d_ecs_v1::Vec3{x:0.0,y:0.0,z:0.0},
    scale: dcl2d_ecs_v1::Vec2{x:1.0,y:1.0},
  };

  dcl2d_ecs_v1::Entity{
    name:"Background".to_string(),
    components:vec![Box::new(renderer),Box::new(transform)],
    ..default()
  }
}

pub fn read_roads_data() -> Result<RoadsData>
{
  let path = "./assets/roads.mp";

  let file = match File::open(&path)
  {
    Ok(v) => v,
    Err(e) => return Err(Box::new(e))
  };
  let reader = BufReader::new(file);
  let mut de = Deserializer::new(reader);
  
  match Deserialize::deserialize(&mut de)
  {
    Ok(result) => Ok(result),
    Err(e) => Err(Box::new(e))
  }

}
