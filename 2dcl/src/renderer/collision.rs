use bevy::prelude::*; 
use bevy_inspector_egui::Inspectable;
use bevy::{prelude::*, sprite::{collide_aabb::collide, Anchor}};
use dcl2d_ecs_v1::{collision_type::CollisionType, components::Trigger};

use serde::{Deserialize, Serialize};
use rmp_serde::*;
use super::scene_loader::{BoxCollider, LevelChangeComponent};

pub const TILE_SIZE: f32 = 1.0;


pub struct CollisionResult
{
  pub hit: bool,
  pub collision_type: CollisionType,
}

#[derive(Default, Clone, Inspectable)]
pub struct CollisionMap
{
    pub collision_locations: Vec<Vec2>,
    pub tile_size: f32
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App)
    {
        app.init_resource::<CollisionMap>()
        .add_startup_system_to_stage(StartupStage::PreStartup,setup);
    }
}

fn setup(mut commands: Commands,)
{
    commands.insert_resource(CollisionMap{tile_size: TILE_SIZE, ..default()});
}


pub fn map_collision_check(
  position: Vec3,
  size: Vec2,
  collision_map: CollisionMap
) -> CollisionResult
{

  for collision_location in collision_map.collision_locations
  {
      let collision = collide(
          Vec3{x:position.x,y:position.y+size.y/2.0,z:0.0},
          size,
          collision_location.extend(0.0),
          Vec2::splat(collision_map.tile_size)
      );

      if collision.is_some()
      {
          return CollisionResult{hit:true,collision_type:CollisionType::Solid};
      }
  }

  return CollisionResult{hit:false,collision_type:CollisionType::Solid};
  
}


pub fn box_collision_check(
  position: Vec3,
  size: Vec2,
  collision_location: Vec3,
  collision_collider: &BoxCollider
) -> CollisionResult
{
  let collision = collide(
          Vec3{x:position.x,y:position.y+size.y/2.0,z:0.0},
          size,
          collision_location + collision_collider.center.extend(0.0),
          collision_collider.size
    );

  if collision.is_some()
  { 
    return CollisionResult{hit:true,collision_type:collision_collider.collision_type.clone()};
  }

  return CollisionResult{hit:false,collision_type:CollisionType::Solid};   
}

pub fn box_collision_query_check(
  position: Vec3,
  size: Vec2,
  box_collision_query: &Query<(&GlobalTransform, &BoxCollider)>
) -> CollisionResult
{
  for (wall,collider) in box_collision_query.iter()
  {
    
    
      let collision = collide(
          Vec3{x:position.x,y:position.y+size.y/2.0,z:0.0},
          size,
          wall.translation() + collider.center.extend(0.0),
          collider.size
      );

      if collision.is_some()
      { 
        return CollisionResult{hit:true,collision_type:collider.collision_type.clone()};
      }  
  }

  return CollisionResult{hit:false,collision_type:CollisionType::Solid};
}



pub fn collision_check(
  position: Vec3,
  size: Vec2,
  box_collision_query: &Query<(&GlobalTransform, &BoxCollider)>,
  collision_map: CollisionMap
) ->CollisionResult
{

  let result = box_collision_query_check(position,size,box_collision_query);
  if result.hit
  {
    return result;
  }
  
  map_collision_check(position,size,collision_map)
}

