use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use super::scene_deserializer::BoxCollider;

pub struct PlayerPlugin;

pub const TILE_SIZE: f32 = 25.0;

#[derive(Component, Inspectable, Default)]
pub struct Player
{
    speed: f32
}

impl Plugin for  PlayerPlugin
{

    fn build(&self, app: &mut App) {
    app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_texture_atlas)
        .add_startup_system(spawn_player)
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
        Vec2::new(121.0,253.0),
         2, 
         1, 
         Vec2::splat(0.0));

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(PlayerTextureAtlas(atlas_handle));

}
fn spawn_player(
    mut commands: Commands, 
    atlas: Res<PlayerTextureAtlas>,)
{
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE,TILE_SIZE*2.0));

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
    .insert(Player
        {
            speed: 3.0
        }).id();

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation = Vec3::new(0.0,0.0,1.0);
    let camera_entity = commands.spawn_bundle(camera_bundle).id();
    commands.entity(player).push_children(&[camera_entity]);
 
}


fn player_movement
(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<(&Transform, &BoxCollider, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
)
{  

    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W)
    {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::S)
    {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A)
    {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::D)
    {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta,0.0,0.0);

    if wall_collision_check(target, &wall_query)
    {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0,y_delta,0.0);

    if wall_collision_check(target, &wall_query)
    {
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<(&Transform, &BoxCollider, Without<Player>)>
) -> bool
{
    
    for (wall,collider, player) in wall_query.iter()
    {
        let collision = collide(
            target_player_pos,
            Vec2::new(TILE_SIZE,TILE_SIZE*2.0) * 0.9,
            wall.translation + collider.center.extend(0.0),
            collider.size
        );

        if collision.is_some()
        {
            return false;
        }
    }

   return true;
}