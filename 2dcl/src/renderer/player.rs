use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use std::{fs, path::PathBuf};
use super::{scene_deserializer::BoxCollider, collision::*, animations::*};

pub struct PlayerPlugin;


pub const PLAYER_SCALE: f32 = 0.5;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_COLLIDER: Vec2 = Vec2::new(10.0,40.0);

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
  
    let texture_atlases_mut_ref = &mut texture_atlases;
    let mut path = PathBuf::new();
    path.push("./assets/player_animation.json");

    let animator = get_animator( path, &assets, texture_atlases_mut_ref);
    let sprite = TextureAtlasSprite::new(0);
    
    let player = commands.spawn_bundle(SpriteSheetBundle{
        sprite,
        texture_atlas: animator.atlas.clone(),
        transform: Transform{
            translation: Vec3::new(0.0,0.0,1.0),
            scale: Vec3::ONE * PLAYER_SCALE * animator.scale,
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
            })
        .id();
    
        let mut camera_bundle = Camera2dBundle::default();
        camera_bundle.transform.translation = Vec3::new(0.0,0.0,100.0);
        let camera_entity = commands.spawn_bundle(camera_bundle).id();
        commands.entity(player).push_children(&[camera_entity]);


        
    let wearables_path = fs::read_dir("./assets/wearables/").unwrap();
    for path in wearables_path {
        
        let path_string =   path.unwrap().path().display().to_string();
        if path_string.ends_with(".json")
        {  
            let mut file_path = PathBuf::new();
            file_path.push(path_string);
            let wearable_animator = get_animator( file_path, &assets,texture_atlases_mut_ref);
            let wearable_sprite = TextureAtlasSprite::new(0);
  
            let wearable = commands.spawn_bundle(SpriteSheetBundle{
                sprite: wearable_sprite,
                texture_atlas: wearable_animator.atlas.clone(),
                transform: Transform{
                    translation: Vec3::new(0.0,0.0,2.0),
                    scale: Vec3::ONE * wearable_animator.scale,
                    ..default()
                },
                ..default()
                })
                .insert(Name::new("Wearable"))
                .insert(wearable_animator)
                .id();
                commands.entity(player).push_children(&[wearable]);

        }

    }

}


fn player_movement
(
    mut player_query: Query<(&mut Player, &mut Transform, &mut Animator, &mut TextureAtlasSprite, &Children)>,
    mut player_children:  Query<(&mut Animator, &mut TextureAtlasSprite, Without<Player>)>,
    box_collision_query: Query<(&Transform, &BoxCollider, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<CollisionMap>,
    time: Res<Time>,

)
{  

    let (player, mut transform, animator, mut sprite, children) = player_query.single_mut();
    
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

        if x_delta > 0.0
        {
            sprite.flip_x = false;
        }
        else if x_delta< 0.0
        {
            sprite.flip_x = true;
        }

        let target = transform.translation + Vec3::new(x_delta,0.0,0.0);

        if collision_check(target, player.collider,&box_collision_query, collision_map.clone())
        {
            transform.translation = target;
        }

        let target = transform.translation + Vec3::new(0.0,y_delta,0.0);

        if collision_check(target, player.collider,&box_collision_query, collision_map.clone())
        {
            transform.translation = target;
        }
    }

    change_animator_state(animator,animation_state.to_string());

    for &child in children.clone().iter() {
            
        if let Ok(mut wearable) = player_children.get_mut(child)
        {
         
            change_animator_state(wearable.0,animation_state.to_string());
            wearable.1.flip_x = sprite.flip_x;
        }
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

