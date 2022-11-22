use bevy::prelude::*; 
use bevy::sprite::collide_aabb::collide;
use dcl2d_ecs_v1::collision_type::CollisionType;


use crate::components::BoxCollider;

pub const TILE_SIZE: f32 = 1.0;


pub struct CollisionResult
{
  pub hit: bool,
  pub collision_type: CollisionType,
  pub entity: Option<Entity>
}

#[derive(Default, Clone)]
pub struct CollisionMap
{
    pub tiles: Vec<CollisionTile>,
    pub tile_size: f32
}

#[derive(Default, Clone)]
pub struct CollisionTile
{
  pub location:Vec2,
  pub colliision_type:CollisionType,
  pub entity: Option<Entity>
  
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
  position: &Vec3,
  size: &Vec2,
  collision_map: CollisionMap
) -> CollisionResult
{

  for tile in collision_map.tiles
  {

    let min = Vec2::new(position.x-size.x/2.0,position.y);
    let max = Vec2::new(position.x+size.x/2.0,position.y+size.y);
    if tile.location.x>min.x 
    && tile.location.x<max.x
    && tile.location.y>min.y
    && tile.location.y<max.y
    {
      return CollisionResult{hit:true,collision_type:tile.colliision_type,entity:tile.entity};
    }
  }
 
  return CollisionResult{hit:false,collision_type:CollisionType::Solid,entity:None};
  
}


pub fn box_collision_check(
  position: &Vec3,
  size: &Vec2,
  collision_location: &Vec3,
  collision_collider: &BoxCollider
) -> CollisionResult
{
  let collision = collide(
          Vec3{x:position.x,y:position.y+size.y/2.0,z:0.0},
          *size,
          *collision_location + collision_collider.center.extend(0.0),
          collision_collider.size
    );

  if collision.is_some()
  { 
    return CollisionResult{hit:true,collision_type:collision_collider.collision_type.clone(),entity:None};
  }

  return CollisionResult{hit:false,collision_type:CollisionType::Solid,entity:None};   
}

