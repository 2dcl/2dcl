use dcl_common::{Result, Parcel};
use rmp_serde::encode::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::path::PathBuf;
use bevy::prelude::*;
use rmp_serde::*;

use super::config::{PARCEL_SIZE_X, PARCEL_SIZE_Y};
//pub fn spawn_road(location: Vec3, )

const ROADS_DATA_MP_FILE:&str = "./assets/roads/roads.mp";

const BACKGROUND_PATH:&str = "road-background.png";
const LEFT_BORDER_PATH:&str = "road-left.png";
const RIGHT_BORDER_PATH:&str = "road-right.png";
const TOP_BORDER_PATH:&str = "road-top.png";
const BOTTOM_BORDER_PATH:&str = "road-bottom.png";
const CLOSED_TOP_LEFT_CORNER_PATH:&str = "road-closed-top-left.png";
const CLOSED_TOP_RIGHT_CORNER_PATH:&str = "road-closed-top-right.png";
const CLOSED_BOTTOM_LEFT_CORNER_PATH:&str = "road-closed-bottom-left.png";
const CLOSED_BOTTOM_RIGHT_CORNER_PATH:&str = "road-closed-bottom-right.png";
const OPEN_TOP_LEFT_CORNER_PATH:&str = "road-open-top-left.png";
const OPEN_TOP_RIGHT_CORNER_PATH:&str = "road-open-top-right.png";
const OPEN_BOTTOM_LEFT_CORNER_PATH:&str = "road-open-bottom-left.png";
const OPEN_BOTTOM_RIGHT_CORNER_PATH:&str = "road-open-bottom-right.png";

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

#[derive(Debug, Default)]
struct RoadCornersData
{
  top_left_corner: CornerType,
  top_right_corner: CornerType,
  bottom_left_corner: CornerType,
  bottom_right_corner: CornerType,
  corners_entities: Vec<dcl2d_ecs_v1::Entity>
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum CornerType
{
  #[default]
  NONE,
  CLOSED,
  OPEN,
  VERTICAL,
  HORIZONTAL,
}

#[derive(Debug, Clone)]
enum CornerPosition
{
  TOP_LEFT,
  TOP_RIGHT,
  BOTTOM_LEFT,
  BOTTOM_RIGHT,
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

 
  

  let mut roads_corner_data = make_corners(roads_data, parcel);
  entities.append(&mut make_background_entities(roads_data, parcel,&roads_corner_data));
  entities.append(&mut make_border_entities(roads_data, parcel));
  entities.append(&mut roads_corner_data.corners_entities);

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
  if !is_road(&Parcel(parcel.0-1,parcel.1), roads_data)
  {
    border.append(&mut make_border_entity(&Border::LEFT));
  }

  if !is_road(&Parcel(parcel.0+1,parcel.1), roads_data)
  {
    border.append(&mut make_border_entity(&Border::RIGHT))
  }

  if !is_road(&Parcel(parcel.0,parcel.1-1), roads_data)
  {
    border.append(&mut make_border_entity(&Border::BOTTOM))
  }

  if !is_road(&Parcel(parcel.0,parcel.1+1), roads_data)
  {
    border.append(&mut make_border_entity(&Border::TOP))
  }
  
  border
}

fn make_border_entity(border: &Border) -> Vec<dcl2d_ecs_v1::Entity>
{

  let mut entities = Vec::new();

  let total_tiles = match border
  {
    Border::LEFT=>PARCEL_SIZE_Y as i32/TILE_SIZE.1,
    Border::RIGHT=>PARCEL_SIZE_Y as i32/TILE_SIZE.1,
    Border::TOP=>PARCEL_SIZE_X as i32/TILE_SIZE.0,
    Border::BOTTOM=>PARCEL_SIZE_X as i32/TILE_SIZE.0,
  };

  for i in 1..total_tiles-1
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
      Border::LEFT=> TILE_SIZE.1*i - PARCEL_SIZE_Y as i32/2,
      Border::RIGHT=> TILE_SIZE.1*i - PARCEL_SIZE_Y as i32/2,
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

fn make_corners(roads_data: &RoadsData, parcel: &Parcel) -> RoadCornersData
{
  let mut corners_data = RoadCornersData::default();

  corners_data.top_left_corner = get_corner_type(roads_data, parcel, CornerPosition::TOP_LEFT);
  match make_corner_entity(&corners_data.top_left_corner,&CornerPosition::TOP_LEFT)
  {
    Some(v) => corners_data.corners_entities.push(v),
    None=>{}
  }
  
  corners_data.top_right_corner = get_corner_type(roads_data, parcel, CornerPosition::TOP_RIGHT);
  match make_corner_entity(&corners_data.top_right_corner,&CornerPosition::TOP_RIGHT)
  {
    Some(v) => corners_data.corners_entities.push(v),
    None=>{}
  }

  corners_data.bottom_left_corner = get_corner_type(roads_data, parcel, CornerPosition::BOTTOM_LEFT);
  match make_corner_entity(&corners_data.bottom_left_corner,&CornerPosition::BOTTOM_LEFT)
  {
    Some(v) => corners_data.corners_entities.push(v),
    None=>{}
  }

  corners_data.bottom_right_corner = get_corner_type(roads_data, parcel, CornerPosition::BOTTOM_RIGHT);
  match make_corner_entity(&corners_data.bottom_right_corner,&CornerPosition::BOTTOM_RIGHT)
  {
    Some(v) => corners_data.corners_entities.push(v),
    None=>{}
  }
  
  corners_data
}

fn get_corner_type(roads_data: &RoadsData, parcel: &Parcel, position: CornerPosition) -> CornerType
{
  let mut corner_type = CornerType::default();

  let x_neighbor_is_road;
  let y_neighbor_is_road;
  let diagonal_is_road;

  match position
  {
    CornerPosition::TOP_LEFT => {
      x_neighbor_is_road =  is_road(&Parcel(parcel.0-1,parcel.1), &roads_data);
      y_neighbor_is_road = is_road(&Parcel(parcel.0,parcel.1+1), &roads_data);
      diagonal_is_road = is_road(&Parcel(parcel.0-1,parcel.1+1), &roads_data);
    },
    CornerPosition::TOP_RIGHT => {
      x_neighbor_is_road =  is_road(&Parcel(parcel.0+1,parcel.1), &roads_data);
      y_neighbor_is_road = is_road(&Parcel(parcel.0,parcel.1+1), &roads_data);
      diagonal_is_road = is_road(&Parcel(parcel.0+1,parcel.1+1), &roads_data);
    },
    CornerPosition::BOTTOM_LEFT => {
      x_neighbor_is_road =  is_road(&Parcel(parcel.0-1,parcel.1), &roads_data);
      y_neighbor_is_road = is_road(&Parcel(parcel.0,parcel.1-1), &roads_data);
      diagonal_is_road = is_road(&Parcel(parcel.0-1,parcel.1-1), &roads_data);
    },
    CornerPosition::BOTTOM_RIGHT =>{      
      x_neighbor_is_road =  is_road(&Parcel(parcel.0+1,parcel.1), &roads_data);
      y_neighbor_is_road = is_road(&Parcel(parcel.0,parcel.1-1), &roads_data);
      diagonal_is_road = is_road(&Parcel(parcel.0+1,parcel.1-1), &roads_data);
    },
  }

  
  if !x_neighbor_is_road && !y_neighbor_is_road
  {
    corner_type = CornerType::CLOSED;
  }
  else if x_neighbor_is_road && y_neighbor_is_road && !diagonal_is_road
  {
    corner_type = CornerType::OPEN;
  }
  else if x_neighbor_is_road && !y_neighbor_is_road
  {
    corner_type = CornerType::HORIZONTAL;
  }
  else if !x_neighbor_is_road && y_neighbor_is_road
  {
    corner_type = CornerType::VERTICAL;
  }

  corner_type
}
fn make_corner_entity(corner_type: &CornerType, position: &CornerPosition) -> Option<dcl2d_ecs_v1::Entity>
{

  let sprite = match position
  {
    CornerPosition::TOP_LEFT => {
      match corner_type
      {
        CornerType::NONE => return None,
        CornerType::CLOSED => CLOSED_TOP_LEFT_CORNER_PATH.to_string(),
        CornerType::OPEN => OPEN_TOP_LEFT_CORNER_PATH.to_string(),
        CornerType::VERTICAL => LEFT_BORDER_PATH.to_string(),
        CornerType::HORIZONTAL => TOP_BORDER_PATH.to_string(),
      }
    },
    CornerPosition::TOP_RIGHT => {
      match corner_type
      {
        CornerType::NONE => return None,
        CornerType::CLOSED => CLOSED_TOP_RIGHT_CORNER_PATH.to_string(),
        CornerType::OPEN => OPEN_TOP_RIGHT_CORNER_PATH.to_string(),
        CornerType::VERTICAL => RIGHT_BORDER_PATH.to_string(),
        CornerType::HORIZONTAL => TOP_BORDER_PATH.to_string(),
      }
    },
    CornerPosition::BOTTOM_RIGHT => {
      match corner_type
      {
        CornerType::NONE => return None,
        CornerType::CLOSED => CLOSED_BOTTOM_RIGHT_CORNER_PATH.to_string(),
        CornerType::OPEN => OPEN_BOTTOM_RIGHT_CORNER_PATH.to_string(),
        CornerType::VERTICAL => RIGHT_BORDER_PATH.to_string(),
        CornerType::HORIZONTAL => BOTTOM_BORDER_PATH.to_string(),
      }
    },
    CornerPosition::BOTTOM_LEFT => {
      match corner_type
      {
        CornerType::NONE => return None,
        CornerType::CLOSED => CLOSED_BOTTOM_LEFT_CORNER_PATH.to_string(),
        CornerType::OPEN => OPEN_BOTTOM_LEFT_CORNER_PATH.to_string(),
        CornerType::VERTICAL => LEFT_BORDER_PATH.to_string(),
        CornerType::HORIZONTAL => BOTTOM_BORDER_PATH.to_string(),
      }
    },
  };

  let location = match position
  {
    CornerPosition::TOP_LEFT => dcl2d_ecs_v1::Vec2{x:PARCEL_SIZE_X as i32 /-2,y:PARCEL_SIZE_Y as i32 /2},
    CornerPosition::TOP_RIGHT => dcl2d_ecs_v1::Vec2{x:PARCEL_SIZE_X as i32 /2,y:PARCEL_SIZE_Y as i32 /2},
    CornerPosition::BOTTOM_RIGHT => dcl2d_ecs_v1::Vec2{x:PARCEL_SIZE_X as i32 /2,y:PARCEL_SIZE_Y as i32 /-2},
    CornerPosition::BOTTOM_LEFT => dcl2d_ecs_v1::Vec2{x:PARCEL_SIZE_X as i32 /-2,y:PARCEL_SIZE_Y as i32 /-2},
  };

  let transform = dcl2d_ecs_v1::components::Transform
  {
    location: location,
    rotation: dcl2d_ecs_v1::Vec3{x:0.0,y:0.0,z:0.0},
    scale: dcl2d_ecs_v1::Vec2{x:1.0,y:1.0},
  };

  let anchor = match position
  {
    CornerPosition::TOP_LEFT => dcl2d_ecs_v1::Anchor::TopLeft,
    CornerPosition::TOP_RIGHT => dcl2d_ecs_v1::Anchor::TopRight,
    CornerPosition::BOTTOM_LEFT => dcl2d_ecs_v1::Anchor::BottomLeft,
    CornerPosition::BOTTOM_RIGHT => dcl2d_ecs_v1::Anchor::BottomRight,
  };

  let renderer = dcl2d_ecs_v1::components::SpriteRenderer
  {
    sprite,
    layer:-1,
    anchor,
    ..default()
  };
  
  Some(dcl2d_ecs_v1::Entity{
    name:"Corner".to_string(),
    components:vec![Box::new(renderer),Box::new(transform)],
    ..default()
  })

}

fn make_background_entities(roads_data: &RoadsData, parcel: &Parcel, corners_data: &RoadCornersData) -> Vec<dcl2d_ecs_v1::Entity>
{
  let mut entities = Vec::new();
  let total_tiles_x = PARCEL_SIZE_X as i32/TILE_SIZE.0;
  let total_tiles_y = PARCEL_SIZE_Y as i32/TILE_SIZE.1;

  let first_x = match is_road(&Parcel(parcel.0-1,parcel.1), roads_data)
  {
    true => 0,
    false => 1,
  };
  let first_y = match is_road(&Parcel(parcel.0,parcel.1-1), roads_data)
  {
    true => 0,
    false => 1,
  };
  let last_x = match is_road(&Parcel(parcel.0+1,parcel.1), roads_data)
  {
    true => total_tiles_x,
    false => total_tiles_x-1,
  };
  let last_y = match is_road(&Parcel(parcel.0,parcel.1+1), roads_data)
  {
    true => total_tiles_y,
    false => total_tiles_y-1,
  };
  for x in first_x..last_x
  {
    for y in first_y..last_y
    {
      if is_corner_at(&corners_data, x, y)
      {   
        continue;
      }

      let renderer = dcl2d_ecs_v1::components::SpriteRenderer
      {
        sprite:BACKGROUND_PATH.to_string(),
        layer:-3,
        anchor: dcl2d_ecs_v1::Anchor::BottomLeft,
        ..default()
      };
    
      let location_x =  TILE_SIZE.0*x - PARCEL_SIZE_X as i32/2;
      let location_y = TILE_SIZE.1*y - PARCEL_SIZE_Y as i32/2;

      let transform = dcl2d_ecs_v1::components::Transform
      {
        location: dcl2d_ecs_v1::Vec2{x:location_x,y:location_y},
        rotation: dcl2d_ecs_v1::Vec3{x:0.0,y:0.0,z:0.0},
        scale: dcl2d_ecs_v1::Vec2{x:1.0,y:1.0},
      };
    
      entities.push(dcl2d_ecs_v1::Entity{
        name:"Background".to_string(),
        components:vec![Box::new(renderer),Box::new(transform)],
        ..default()
      });
    }
  }

  entities
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

pub fn is_road(parcel: &Parcel, roads_data: &RoadsData)-> bool
{
  roads_data.parcel_map.get(&(parcel.0,parcel.1)).is_some()
}

fn is_corner_at(corners_data:&RoadCornersData,x:i32,y:i32) -> bool
{
  let total_tiles_x = PARCEL_SIZE_X as i32/TILE_SIZE.0 -1;
  let total_tiles_y = PARCEL_SIZE_Y as i32/TILE_SIZE.1 -1;

  if x==0 && y ==0
  { 
    return corners_data.bottom_left_corner != CornerType::NONE
  }

  if x==0 && y == total_tiles_y
  {    
    return corners_data.top_left_corner != CornerType::NONE
  }

  if x==total_tiles_x && y == 0
  {    
    return corners_data.bottom_right_corner != CornerType::NONE
  }
  
  if x==total_tiles_x && y == total_tiles_y
  {
    return corners_data.top_right_corner != CornerType::NONE
  }
  
  false
}

