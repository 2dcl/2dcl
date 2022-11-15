use bevy::{prelude::*, sprite::{collide_aabb::collide, Anchor}};
use bevy_inspector_egui::Inspectable;
use dcl2d_ecs_v1::collision_type::CollisionType;
use super::{scene_loader::BoxCollider, collision::{*, self}, animations::*};

pub struct PlayerPlugin;


pub const PLAYER_SCALE: f32 = 1.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_COLLIDER: Vec2 = Vec2::new(20.0,5.0);

#[derive(Component, Default, Inspectable)]
pub struct Player
{
    speed: f32,
    collider: Vec2,
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
        .insert(Player
            {
                speed: PLAYER_SPEED,
                collider: PLAYER_COLLIDER 
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
    mut player_query: Query<(&mut Player, &mut Transform, &mut Animator, &mut TextureAtlasSprite)>,
   // mut player_renderer_query:  Query<(&mut Animator, &mut TextureAtlasSprite, Without<Player>)>,
    box_collision_query: Query<(&GlobalTransform, &BoxCollider)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<CollisionMap>,
    time: Res<Time>

)
{  

    let (player, mut transform, mut animator, mut texture_atlas) = player_query.single_mut();
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

        let collision_result = collision_check(target, player.collider,&box_collision_query, collision_map.clone());
        
        if !collision_result.hit
        {
          transform.translation = target;
        }
 

        let target = transform.translation + Vec3::new(0.0,y_delta,y_delta * -1.0);

        let collision_result = collision_check(target, player.collider,&box_collision_query, collision_map.clone());
        
        if !collision_result.hit
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
