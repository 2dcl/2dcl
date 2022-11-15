use bevy::{prelude::*, sprite::Anchor};
use dcl2d_ecs_v1::{collision_type::CollisionType};
use dcl_common::Parcel;
use super::{scene_loader::{BoxCollider, LevelChangeComponent}, collision::*, animations::*};

pub struct PlayerPlugin;


pub const PLAYER_SCALE: f32 = 1.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_COLLIDER: Vec2 = Vec2::new(20.0,5.0);


#[derive(Component)]
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

  let mut current_path = std::env::current_exe().unwrap();
  current_path.pop();
  current_path.push("assets");
  current_path.push("player.json");

    let animator = get_animator(current_path, &assets,  &mut texture_atlases).unwrap();
   
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

        camera_bundle.projection.scale = 0.5;
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

    let (mut player, mut transform, mut animator, mut texture_atlas) = player_query.single_mut();
    let mut y_delta = 0.0;
    let mut animation_state = "Idle";

    if keyboard.pressed(KeyCode::W)
    {
        y_delta += player.speed * PLAYER_SCALE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::S)
    {
        y_delta -= player.speed * PLAYER_SCALE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A)
    {
        x_delta -= player.speed * PLAYER_SCALE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::D)
    {
        x_delta += player.speed * PLAYER_SCALE * time.delta_seconds();
    }

    let mut walking = false;
    if x_delta.abs()>0.0 || y_delta.abs()>0.0
    {
        walking = true;
    }

    if walking
    {
      animation_state = "Run";

      let target = transform.translation + Vec3::new(x_delta,0.0,0.0);

      if !check_player_collision (
        player.as_mut(),
        target,
        &box_collision_query,
        &trigger_query,
        collision_map.clone() )
      {
        transform.translation = target;
      }
      
      let target = transform.translation + Vec3::new(0.0,y_delta,y_delta * -1.0);

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

    if x_delta > 0.0
    {
        texture_atlas.flip_x = false;
    }
    else if x_delta< 0.0
    {
        texture_atlas.flip_x = true;
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

