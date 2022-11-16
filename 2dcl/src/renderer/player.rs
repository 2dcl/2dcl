use bevy::{prelude::*, sprite::Anchor};
use dcl2d_ecs_v1::{collision_type::CollisionType};
use dcl_common::Parcel;
use super::{scene_loader::{BoxCollider, LevelChangeComponent}, collision::*, animations::*};
use crate::renderer::config::*;

pub struct PlayerPlugin;


#[derive(Component, Debug)]
pub struct PlayerComponent
{
    speed: f32,
    collider_size: Vec2,
    pub current_level: usize,
    pub current_parcel: Parcel,
}


impl Plugin for  PlayerPlugin
{

    fn build(&self, app: &mut App) {
    app
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        ;
    }
}

fn spawn_player(
    mut commands: Commands, 
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{   

  let mut current_path = std::env::current_exe().unwrap_or_default();
  current_path.pop();
  current_path.push("assets");
  current_path.push("player.json");

    let animator = get_animator(current_path, &assets,  &mut texture_atlases);
   
    if animator.is_err()
    {
      println!("{}",animator.unwrap_err());
      return;
    }

    let animator = animator.unwrap();
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.anchor = Anchor::BottomCenter;

    //Spawning Entity
    let player = commands.spawn_bundle(SpriteSheetBundle{
        sprite,
        texture_atlas: animator.atlas.clone(),
        transform: Transform{
            scale: Vec3::ONE * PLAYER_SCALE * animator.scale,
            ..default()
        },
        ..default()
        })
        .insert(animator)
        .insert(Name::new("Player"))
        .insert(PlayerComponent
            {
                speed: PLAYER_SPEED,
                collider_size: PLAYER_COLLIDER,
                current_level: 0,
                current_parcel:Parcel(0,0)
            })
        .id();
    
        let mut camera_bundle = Camera2dBundle::new_with_far(10000.0);
        camera_bundle.transform = Transform::from_translation(Vec3{x:0.0,y:0.0,z:5000.0});

        camera_bundle.projection.scale = CAMERA_SCALE;
        let camera_entity = commands.spawn_bundle(camera_bundle).id();
        
        commands.entity(player).add_child(camera_entity);
    
}


fn player_movement
(
    mut player_query: Query<(&mut PlayerComponent, &mut Transform, &mut Animator, &mut TextureAtlasSprite)>,
    box_collision_query: Query<(Entity, &GlobalTransform, &BoxCollider,Without<PlayerComponent>)>,
    trigger_query: Query<(Entity, &LevelChangeComponent, Without<PlayerComponent>)>,  
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<CollisionMap>,
    time: Res<Time>

)
{  

    let result = player_query.get_single_mut();
    
    if result.is_err()
    {
      println!("{}",result.unwrap_err());
      return;
    }

    let (mut player, mut transform, mut animator, mut texture_atlas) = result.unwrap();

    let mut animation_state = "Idle";
    let mut movement_input = get_movment_axis_input(&keyboard);

    movement_input = movement_input.normalize();
    movement_input = movement_input * player.speed * PLAYER_SCALE * time.delta_seconds();
    
    let mut walking = false;
    if movement_input.length() > 0f32
    {
        walking = true;
    }

    if walking
    {
      animation_state = "Run";

      let target = transform.translation + Vec3::new(movement_input.x,0.0,0.0);

      if !check_player_collision (
        player.as_mut(),
        target,
        &box_collision_query,
        &trigger_query,
        collision_map.clone() )
      {
        transform.translation = target;
      }
      
      let target = transform.translation + Vec3::new(0.0,movement_input.y,movement_input.y * -1.0);

      if !check_player_collision (
        player.as_mut(),
        target,
        &box_collision_query,
        &trigger_query,
        collision_map.clone() )
      {
        transform.translation = target;
      }
    }

    if movement_input.x > 0.0
    {
        texture_atlas.flip_x = true;
    }
    else if movement_input.x< 0.0
    {
        texture_atlas.flip_x = false;
    }
    change_animator_state(animator.as_mut(),animation_state.to_string()); 

}


fn check_player_collision(
  player: &mut PlayerComponent,
  target_location: Vec3,
  box_collision_query: &Query<(Entity, &GlobalTransform, &BoxCollider,Without<PlayerComponent>)>,
  trigger_query: &Query<(Entity, &LevelChangeComponent, Without<PlayerComponent>)>,  
  collision_map: CollisionMap,
) -> bool
{
  let mut blocked = false;
  for (collision_entity, collision_transform, collider, _player) in box_collision_query.iter()
  {
    let collision_result = box_collision_check (
      target_location,player.collider_size, 
      collision_transform.translation(), 
      collider);
   
    if collision_result.hit
    {
      if collision_result.collision_type == CollisionType::Solid
      {
        blocked = true;
      }
      else
      {
      
        for (trigger_entity, trigger, _player) in trigger_query.iter()
        {
          if trigger_entity == collision_entity
          {
            player.current_level = trigger.level;
            break;
          }
        }
      }
    } 
  }

  if blocked
  {
    return true;
  }
  
  let collision_result =  map_collision_check (
    target_location,
    player.collider_size,
    collision_map);
  if collision_result.hit
  {
    return true;
  } 
  
  return false;
}


fn get_movment_axis_input(
  keyboard: &Res<Input<KeyCode>>,
) -> Vec3
{
  let mut movement_input = Vec3::default();

  if keyboard.pressed(KeyCode::W)
  {
    movement_input.y+= 1f32;
  }

  if keyboard.pressed(KeyCode::S)
  {
    movement_input.y-= 1f32;
  }

  if keyboard.pressed(KeyCode::D)
  {
    movement_input.x+= 1f32;
  }

  if keyboard.pressed(KeyCode::A)
  {
    movement_input.x-= 1f32;
  }

  movement_input
  
}