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
    level_change_stack: Vec<LevelChangeStackData>,
    pub current_level: usize,
    pub current_parcel: Parcel,
}

#[derive(Debug)]
struct LevelChangeStackData
{
  location: Vec3,
  level_id: usize,
}


impl Plugin for  PlayerPlugin
{

    fn build(&self, app: &mut App) {
    app
        .add_startup_system(spawn_player)
        .add_system(player_interact)
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
            translation: Vec3::new(-100.0, 0.0, 0.0),
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
                current_parcel:Parcel(0,0),
                level_change_stack: vec![]
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
    box_collision_query: Query<(&GlobalTransform, &BoxCollider,Without<PlayerComponent>)>,
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

      let mut target = transform.translation + Vec3::new(movement_input.x,0.0,0.0);

      if !check_player_collision (
        player.as_mut(),
        &target,
        &box_collision_query,
        collision_map.clone()
      )
      {
        transform.translation = target;
      }
      
      target = transform.translation + Vec3::new(0.0,movement_input.y,movement_input.y * -1.0);

      if !check_player_collision (
        player.as_mut(),
        &target,
        &box_collision_query,
        collision_map.clone() 
      )
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



fn player_interact(
  mut player_query: Query<(&mut PlayerComponent, &mut Transform)>,
  level_change_query: Query<(&GlobalTransform, &BoxCollider, &LevelChangeComponent)>,
  keyboard: Res<Input<KeyCode>>,
)
{
  let result = player_query.get_single_mut();
    
  if result.is_err()
  {
    println!("{}",result.unwrap_err());
    return;
  }
  
  let (mut player, mut transform) = result.unwrap();

  if keyboard.pressed(KeyCode::E)
  {
    enter_level(&mut player,&mut transform, &level_change_query);
  }

  if keyboard.pressed(KeyCode::Escape)
  {
    exit_level(&mut player,&mut transform);
  }
}


fn enter_level( 
  player: &mut PlayerComponent,
  player_transform: &mut Transform,
  level_change_query: &Query<(&GlobalTransform, &BoxCollider, &LevelChangeComponent)>,
)
{
  for (collision_transform, collider, level_change) in level_change_query.iter()
  {
    let collision_result = box_collision_check (
      &player_transform.translation,
      &player.collider_size, 
      &collision_transform.translation(), 
      collider);

      if collision_result.hit && collision_result.collision_type == CollisionType::Trigger
      {
        let level_change_stack_data = LevelChangeStackData{
          level_id: player.current_level,
          location: player_transform.translation
        };

        player.current_level = level_change.level;
        player.level_change_stack.push(level_change_stack_data);
        player_transform.translation = level_change.spawn_point.extend(level_change.spawn_point.y*-1f32);
        if level_change.level==0
        {
          player.level_change_stack.clear();
        }
      }
  }
}


fn exit_level(
  player: &mut PlayerComponent,
  transform: &mut Transform,
)
{
  match player.level_change_stack.pop()
  {
    Some(data) => {
      transform.translation = data.location;
      player.current_level = data.level_id;
    }
    None => {}
  }
  
}


fn check_player_collision(
  player: &mut PlayerComponent,
  target_location: &Vec3,
  box_collision_query: &Query<(&GlobalTransform, &BoxCollider,Without<PlayerComponent>)>,
  collision_map: CollisionMap,
) -> bool
{

  for (collision_transform, collider, _player) in box_collision_query.iter()
  {
    let collision_result = box_collision_check (
      target_location,
      &player.collider_size, 
      &collision_transform.translation(), 
      collider);
   
    if collision_result.hit
    {
      if collision_result.collision_type == CollisionType::Solid
      {
        return true;
      }
    } 
  }
  
  let collision_result =  map_collision_check (
    target_location,
    &player.collider_size,
    collision_map);
  if collision_result.hit
  {
    return true;
  } 
  
  false
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