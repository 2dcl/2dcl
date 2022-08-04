use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use super::{scene_deserializer::BoxCollider, collision::*, animations::*};

pub struct PlayerPlugin;


pub const PLAYER_SCALE: f32 = 0.5;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_COLLIDER: Vec2 = Vec2::new(50.0,100.0);


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
  
    let animator = get_animator( "./assets/player_animation.json".to_string(), assets,texture_atlases);
    let mut sprite = TextureAtlasSprite::new(0);

   
    let player = commands.spawn_bundle(SpriteSheetBundle{
        sprite,
        texture_atlas: animator.atlas.clone(),
        transform: Transform{
            translation: Vec3::new(0.0,0.0,5.0),
            scale: Vec3::ONE * PLAYER_SCALE,
            ..default()
        },
        ..default()
        })
        .insert(Name::new("Player"))
        .insert(animator)
        .insert(Player
            {
                speed: PLAYER_SPEED,
                collider: PLAYER_COLLIDER 
            }).id();
    
        let mut camera_bundle = Camera2dBundle::default();
        camera_bundle.transform.translation = Vec3::new(0.0,0.0,1.0);
        let camera_entity = commands.spawn_bundle(camera_bundle).id();
        commands.entity(player).push_children(&[camera_entity]);
}


fn player_movement
(
    mut player_query: Query<(&mut Player, &mut Transform, &mut Animator, &mut TextureAtlasSprite,)>,
    wall_query: Query<(&Transform, &BoxCollider, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<CollisionMap>,
    time: Res<Time>
)
{  

    let (mut player, mut transform,  animator, mut sprite) = player_query.single_mut();

    let mut y_delta = 0.0;
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
        change_animator_state(animator, "Run".to_string());
        if x_delta > 0.0
        {
            sprite.flip_x = false;
        }
        else if x_delta< 0.0
        {
            sprite.flip_x = true;
        }

        let target = transform.translation + Vec3::new(x_delta,0.0,0.0);

        if collision_check(target, player.collider,&wall_query, collision_map.clone())
        {
            transform.translation = target;
        }

        let target = transform.translation + Vec3::new(0.0,y_delta,0.0);

        if collision_check(target, player.collider,&wall_query, collision_map.clone())
        {
            transform.translation = target;
        }
    }
    else
    {
        
        change_animator_state(animator,"Idle".to_string());
    }

    

}

fn collision_check(
    target_player_pos: Vec3,
    target_player_collider: Vec2,
    box_collision_query: &Query<(&Transform, &BoxCollider, Without<Player>)>,
    collision_map: CollisionMap
) -> bool
{
    //box colliders
    for (wall,collider, _player) in box_collision_query.iter()
    {
        let collision = collide(
            target_player_pos,
            target_player_collider,
            wall.translation + collider.center.extend(0.0),
            collider.size
        );

        if collision.is_some()
        {
            return false;
        }
    }
    
    for collision_location in collision_map.collision_locations
    {
        //Alpha colliders
        let collision = collide(
            target_player_pos,
            target_player_collider,
            collision_location.extend(0.0), //wall.translation + collider.center.extend(0.0),
            Vec2::splat(collision_map.tile_size)
        );

        if collision.is_some()
        {
            return false;
        }

    }

    return true;

}

