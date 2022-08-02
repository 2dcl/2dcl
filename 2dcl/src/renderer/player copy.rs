use bevy::{prelude::*, sprite::collide_aabb::collide};
//use bevy_inspector_egui::Inspectable;

use super::{scene_deserializer::BoxCollider, collision::*};

pub struct PlayerPlugin;


pub const PLAYER_SCALE: f32 = 0.5;
pub const PLAYER_SPEED: f32 = 300.0;
pub const TEXTURE_ATLAS_TILE_SIZE: [f32;2] = [96.0,105.0];
pub const ANIMATION_PLAY_RATE: f32 = 15.0;

pub const LEFT_IDLE_ANIMATION_START: usize = 10;
pub const LEFT_IDLE_ANIMATION_FRAME_LENGTH: usize = 3;
pub const RIGHT_IDLE_ANIMATION_START: usize = 30;
pub const RIGHT_IDLE_ANIMATION_FRAME_LENGTH: usize = 3;
pub const LEFT_WALKING_ANIMATION_START: usize = 50;
pub const LEFT_WALKING_ANIMATION_FRAME_LENGTH: usize = 10;
pub const RIGHT_WALKING_ANIMATION_START: usize = 70;
pub const RIGHT_WALKING_ANIMATION_FRAME_LENGTH: usize = 10;

#[derive(Component, Default)]
pub struct Player
{
    speed: f32,
    animation_state: AnimationState,
    is_facing_right: bool
}


#[derive( Default)]
enum AnimationState
{
    #[default]
    Idle,
    Walking
}

impl Plugin for  PlayerPlugin
{

    fn build(&self, app: &mut App) {
    app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_texture_atlas)
        .add_startup_system(spawn_player)
        .add_system(animate_sprite)
        .add_system(player_movement)
        ;
    }
}

struct PlayerTextureAtlas(Handle<TextureAtlas>);

fn load_texture_atlas(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
)
{
    let texture = assets.load("player.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        texture, 
        Vec2::new(TEXTURE_ATLAS_TILE_SIZE[0],TEXTURE_ATLAS_TILE_SIZE[1]),
         10, 
         8, 
         Vec2::splat(1.0),
        Vec2::ZERO);

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(PlayerTextureAtlas(atlas_handle));

}
fn spawn_player(
    mut commands: Commands, 
    atlas: Res<PlayerTextureAtlas>,)
{
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(TEXTURE_ATLAS_TILE_SIZE[0],TEXTURE_ATLAS_TILE_SIZE[1])*PLAYER_SCALE);

    let player = commands.spawn_bundle(SpriteSheetBundle{
    sprite,
    texture_atlas: atlas.0.clone(),
    transform: Transform{
        translation: Vec3::new(0.0,0.0,5.0),
        ..default()
    },
    ..default()
    })
    .insert(Name::new("Player"))
    .insert(AnimationTimer(Timer::from_seconds(1.0/ANIMATION_PLAY_RATE, true)))
    .insert(Player
        {
            speed: PLAYER_SPEED,
            animation_state: AnimationState::Idle,
            is_facing_right: true
        }).id();

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation = Vec3::new(0.0,0.0,1.0);
    let camera_entity = commands.spawn_bundle(camera_bundle).id();
    commands.entity(player).push_children(&[camera_entity]);
 
}


fn player_movement
(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<(&Transform, &BoxCollider, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<CollisionMap>,
    time: Res<Time>
)
{  

    let (mut player, mut transform) = player_query.single_mut();

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
        if x_delta > 0.0
        {
            player.is_facing_right = true;
        }
        else if x_delta< 0.0
        {
            player.is_facing_right = false;
        }

        let target = transform.translation + Vec3::new(x_delta,0.0,0.0);

        if collision_check(target, &wall_query, collision_map.clone())
        {
            transform.translation = target;
        }

        let target = transform.translation + Vec3::new(0.0,y_delta,0.0);

        if collision_check(target, &wall_query, collision_map.clone())
        {
            transform.translation = target;
        }
        
        player.animation_state = AnimationState::Walking;
    }
    else
    {
        player.animation_state = AnimationState::Idle;
    }

    

}

fn collision_check(
    target_player_pos: Vec3,
    box_collision_query: &Query<(&Transform, &BoxCollider, Without<Player>)>,
    collision_map: CollisionMap
) -> bool
{
    //box colliders
    for (wall,collider, _player) in box_collision_query.iter()
    {
        let collision = collide(
            target_player_pos,
            Vec2::new(TEXTURE_ATLAS_TILE_SIZE[0],TEXTURE_ATLAS_TILE_SIZE[1]) * PLAYER_SCALE * 0.9,
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
            Vec2::new(TEXTURE_ATLAS_TILE_SIZE[0],TEXTURE_ATLAS_TILE_SIZE[1]) * PLAYER_SCALE * 0.9,
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

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Player
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, player) in &mut query.iter_mut() 
    {
      
       let animation_start: usize;
       let animation_end: usize;

        match player.animation_state
        {

            AnimationState::Idle => { 
                if player.is_facing_right
                {
                    animation_start = RIGHT_IDLE_ANIMATION_START;
                    animation_end = RIGHT_IDLE_ANIMATION_START + RIGHT_IDLE_ANIMATION_FRAME_LENGTH;
                }
                else
                {
                    animation_start = LEFT_IDLE_ANIMATION_START;
                    animation_end = LEFT_IDLE_ANIMATION_START + LEFT_IDLE_ANIMATION_FRAME_LENGTH;
                }
            }

            AnimationState::Walking => { 
                if player.is_facing_right
                {
                    animation_start = RIGHT_WALKING_ANIMATION_START;
                    animation_end = RIGHT_WALKING_ANIMATION_START + RIGHT_WALKING_ANIMATION_FRAME_LENGTH;
                }
                else
                {
                    animation_start = LEFT_WALKING_ANIMATION_START;
                    animation_end = LEFT_WALKING_ANIMATION_START + LEFT_WALKING_ANIMATION_FRAME_LENGTH;
                }
                
            }
            
        }

        if sprite.index < animation_start || sprite.index >= animation_end
        {
            sprite.index = animation_start;
            timer.reset();
        }
        else{
            timer.tick(time.delta());
            if timer.just_finished() {

                let mut new_index = sprite.index + 1;
                
                if new_index >= animation_end
                {
                    new_index = animation_start;
                }

                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (new_index) % texture_atlas.textures.len();
            }
        }
    }
}