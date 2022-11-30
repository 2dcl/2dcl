
use dcl_common::{Result, Parcel};
use rmp_serde::encode::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::path::PathBuf;
use bevy::prelude::*;
use rmp_serde::*;
use super::config::{PARCEL_SIZE_X, PARCEL_SIZE_Y};
use super::scene_loader::parcel_to_world_location;
//pub fn spawn_road(location: Vec3, )

const ROADS_DATA_MP_FILE:&str = "./assets/roads/roads.mp";

const BACKGROUND_PATH:&str = "road-background.png";
const LEFT_BORDER_PATH:&str = "road-left.png";
const RIGHT_BORDER_PATH:&str = "road-right.png";
const TOP_BORDER_PATH:&str = "road-top.png";
const BOTTOM_BORDER_PATH:&str = "road-bottom.png";
const TOP_LEFT_CORNER_PATH:&str = "road-background";
const TOP_RIGHT_CORNER_PATH:&str = "road-background";
const BOTTOM_LEFT_CORNER_PATH:&str = "road-background";
const BOTTOM_RIGHT_CORNER_PATH:&str = "road-background";

const TILE_SIZE:(i32,i32) = (64,64);

#[derive(Debug, Clone, Default)]
pub struct RoadsData
{  
  pub parcel_map: HashMap<(i16, i16), ()>
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SerializableRoadsData
{
  parcels:Vec<Parcel>
}

enum Border
{
  LEFT,
  RIGHT,
  TOP,
  BOTTOM
}

enum Corner
{
  TOP_LEFT,
  TOP_RIGHT,
  BOTTOM_LEFT,
  BOTTOM_RIGHT
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
  match read_roads_data()
  {
    Ok(roads_data) => commands.insert_resource(roads_data),
    Err(e) => println!("error:{}",e)
  }


}

pub fn make_road_scene(roads_data: &RoadsData, parcel: &Parcel) ->  (Result<dcl2d_ecs_v1::Scene>, PathBuf)
{

  let mut entities: Vec<dcl2d_ecs_v1::Entity> = Vec::new();

  entities.push(make_background_entity());
  entities.append(&mut make_border_entities(roads_data, parcel));

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

  let mut path = PathBuf::default();
  path.push("..");
  path.push("assets");
  path.push("roads");
  
  (Ok(scene),path)
}


fn make_border_entities(roads_data: &RoadsData, parcel: &Parcel) -> Vec<dcl2d_ecs_v1::Entity>
{
  
  let mut border = Vec::new();
  match roads_data.parcel_map.get(&(parcel.0-1,parcel.1))
  {
    Some(_)=> {},
    None => { border.append(&mut make_border(&Border::LEFT))}
  }

  match roads_data.parcel_map.get(&(parcel.0+1,parcel.1))
  {
    Some(_)=> {},
    None => { border.append(&mut make_border(&Border::RIGHT))}
  }

  match roads_data.parcel_map.get(&(parcel.0,parcel.1-1))
  {
    Some(_)=> {},
    None => { border.append(&mut make_border(&Border::BOTTOM))}
  }

  match roads_data.parcel_map.get(&(parcel.0,parcel.1+1))
  {
    Some(_)=> {},
    None => { border.append(&mut make_border(&Border::TOP))}
  }
  
  border
}

fn make_border(border: &Border) -> Vec<dcl2d_ecs_v1::Entity>
{

  let mut entities = Vec::new();

  let total_tiles = match border
  {
    Border::LEFT=>PARCEL_SIZE_Y as i32/TILE_SIZE.1,
    Border::RIGHT=>PARCEL_SIZE_Y as i32/TILE_SIZE.1,
    Border::TOP=>PARCEL_SIZE_X as i32/TILE_SIZE.0,
    Border::BOTTOM=>PARCEL_SIZE_X as i32/TILE_SIZE.0,
  };

  for i in 0..total_tiles
  {
    let location_x = match border
    {
      Border::LEFT=> PARCEL_SIZE_X as i32 /-2,
      Border::RIGHT=> PARCEL_SIZE_X as i32 /2,
      Border::TOP=> TILE_SIZE.0*i - PARCEL_SIZE_X as i32/2,
      Border::BOTTOM=> TILE_SIZE.0*i - PARCEL_SIZE_X as i32/2
    };

    let location_y = match border
    {
      Border::LEFT=> TILE_SIZE.0*i - PARCEL_SIZE_Y as i32/2,
      Border::RIGHT=> TILE_SIZE.0*i - PARCEL_SIZE_Y as i32/2,
      Border::TOP=> PARCEL_SIZE_Y as i32 /2,
      Border::BOTTOM=> PARCEL_SIZE_Y as i32 /-2
    };
    
    let transform = dcl2d_ecs_v1::components::Transform
    {
      location: dcl2d_ecs_v1::Vec2{x:location_x,y:location_y},
      rotation: dcl2d_ecs_v1::Vec3{x:0.0,y:0.0,z:0.0},
      scale: dcl2d_ecs_v1::Vec2{x:1.0,y:1.0},
    };

    let sprite = match border
    {
      Border::LEFT=> LEFT_BORDER_PATH.to_string(),
      Border::RIGHT=> RIGHT_BORDER_PATH.to_string(),
      Border::TOP=> TOP_BORDER_PATH.to_string(),
      Border::BOTTOM=> BOTTOM_BORDER_PATH.to_string(),
    };

    let anchor = match border
    {
      Border::LEFT=> dcl2d_ecs_v1::Anchor::BottomLeft,
      Border::RIGHT=> dcl2d_ecs_v1::Anchor::BottomRight,
      Border::TOP=> dcl2d_ecs_v1::Anchor::TopLeft,
      Border::BOTTOM=> dcl2d_ecs_v1::Anchor::BottomLeft,
    };

    let renderer = dcl2d_ecs_v1::components::SpriteRenderer
    {
      sprite,
      layer:-2,
      anchor,
      ..default()
    };
    

    let entity = dcl2d_ecs_v1::Entity{
      name:"Border".to_string(),
      components:vec![Box::new(renderer),Box::new(transform)],
      ..default()
    };

    entities.push(entity);

  }

  entities
}

fn make_background_entity() -> dcl2d_ecs_v1::Entity
{
  let renderer = dcl2d_ecs_v1::components::SpriteRenderer
  {
    sprite:BACKGROUND_PATH.to_string(),
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

  let file = match File::open(&ROADS_DATA_MP_FILE)
  {
    Ok(v) => v,
    Err(e) => return Err(Box::new(e))
  };
  let reader = BufReader::new(file);
  let mut de = Deserializer::new(reader);
  
  let serializable_roads_data: SerializableRoadsData = match Deserialize::deserialize(&mut de)
  {
    Ok(result) => result,
    Err(e) => return Err(Box::new(e))
  };

  let mut roads_data =  RoadsData::default();

  for parcel in serializable_roads_data.parcels
  {
    roads_data.parcel_map.insert((parcel.0,parcel.1),());
  }

  Ok(roads_data)

}

pub fn update_roads_data(new_roads_data: &RoadsData) -> Result<()>
{
  let mut serializable_roads_data = SerializableRoadsData::default();
  
  for key in new_roads_data.parcel_map.keys()
  {
    serializable_roads_data.parcels.push(Parcel(key.0,key.1));
  }

  let mut buf: Vec<u8> = Vec::new();
  match serializable_roads_data.serialize(&mut Serializer::new(&mut buf))
  {
    Ok(_)=>{},
    Err(e)=> return Err(Box::new(e))
  }

  let mut file = match File::create(&ROADS_DATA_MP_FILE)
  {
    Ok(v) => v,
    Err(e) => return Err(Box::new(e))
  };

  match file.write_all(&buf)
  {
    Ok(v) => Ok(v),
    Err(e) => Err(Box::new(e))
  }
}


pub fn remove_road_at_parcel(parcel: &Parcel, roads_data: &mut RoadsData)-> Result<()>
{
  roads_data.parcel_map.remove(&(parcel.0,parcel.1));
  update_roads_data(roads_data)

}

pub fn add_road_at_parcel(parcel: &Parcel, roads_data: &mut RoadsData)-> Result<()>
{
  roads_data.parcel_map.insert((parcel.0,parcel.1),());
  update_roads_data(roads_data)
}