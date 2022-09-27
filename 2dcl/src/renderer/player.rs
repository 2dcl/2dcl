use bevy::{prelude::*, sprite::{collide_aabb::collide, Anchor}};
use bevy_inspector_egui::Inspectable;
use super::{scene_deserializer::BoxCollider, collision::*, animations::*};

pub struct PlayerPlugin;


pub const PLAYER_SCALE: f32 = 1.0;
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
    let animator = get_animator( "./2dcl/assets/player.json", &assets, texture_atlases_mut_ref).unwrap();
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.anchor = Anchor::BottomCenter;

    //Spawning Entity
    let player = commands.spawn()
        .insert(Name::new("Player"))
        .insert(Visibility{is_visible: true})
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Transform::default())
        .insert(Player
            {
                speed: PLAYER_SPEED,
                collider: PLAYER_COLLIDER 
            })
        .id();
        
    let player_sprite = commands.spawn_bundle(SpriteSheetBundle{
        sprite,
        texture_atlas: animator.atlas.clone(),
        transform: Transform{
            scale: Vec3::ONE * PLAYER_SCALE * animator.scale,
            ..default()
        },
        ..default()
        })
        .insert(Name::new("Player - sprite renderer"))
        .insert(animator)
        .id();
    
        let mut camera_bundle = Camera2dBundle::new_with_far(1000.0);
        camera_bundle.transform = Transform::from_translation(Vec3{x:0.0,y:0.0,z:500.0});
        camera_bundle.projection.scale = 0.5;
        let camera_entity = commands.spawn_bundle(camera_bundle).id();
        
        commands.entity(player).add_child(camera_entity);
        commands.entity(player).add_child(player_sprite);


    
}


fn player_movement
(
    mut player_query: Query<(&mut Player, &mut Transform, &Children)>,
    mut player_renderer_query:  Query<(&mut Animator, &mut TextureAtlasSprite, Without<Player>)>,
    box_collision_query: Query<(&Transform, &BoxCollider, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<CollisionMap>,
    time: Res<Time>

)
{  

    let (player, mut transform, children) = player_query.single_mut();
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


    let mut flip_x = false;

    if walking
    {
        animation_state = "Run";

        if x_delta< 0.0
        {
            flip_x = true;
        }

        let target = transform.translation + Vec3::new(x_delta,0.0,0.0);

        if collision_check(target, player.collider,&box_collision_query, collision_map.clone())
        {
            transform.translation = target;
        }

        let target = transform.translation + Vec3::new(0.0,y_delta,y_delta * -1.0);

        if collision_check(target, player.collider,&box_collision_query, collision_map.clone())
        {
            transform.translation = target;
        }
    }

    for &child in children.clone().iter() {
               
        if let Ok(mut sprite_renderer) = player_renderer_query.get_mut(child)
        {
         
            change_animator_state(sprite_renderer.0,animation_state.to_string());
            sprite_renderer.1.flip_x = flip_x;
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
              Vec3{x:target_player_pos.x,y:target_player_pos.y+target_player_collider.y/2.0,z:0.0},
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

