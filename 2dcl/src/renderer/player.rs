use super::scene_loader::parcel_to_world_location;
use super::transparency::{update_overlapping_elements, update_transparency_on_top_of_player};
use super::{animations::*, collision::*};
use crate::renderer::config::*;
use crate::{components, resources};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::Anchor};
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;

pub struct PlayerPlugin;

#[derive(Debug)]
pub struct LevelChangeStackData {
    location: Vec3,
    level_id: usize,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_interact)
            .add_system(update_transparency_on_top_of_player)
            .add_system(update_overlapping_elements)
            .add_system(player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut player_animator_path = std::env::current_exe().unwrap_or_default();
    player_animator_path.pop();
    player_animator_path.push("assets");
    player_animator_path.push("player.json");

    let player_animator = get_animator(player_animator_path, &assets, &mut texture_atlases);

    if player_animator.is_err() {
        println!("{}", player_animator.unwrap_err());
        return;
    }

    let mut interact_animator_path = std::env::current_exe().unwrap_or_default();
    interact_animator_path.pop();
    interact_animator_path.push("assets");
    interact_animator_path.push("interact.json");

    let interact_animator = get_animator(interact_animator_path, &assets, &mut texture_atlases);

    if interact_animator.is_err() {
        println!("{}", interact_animator.unwrap_err());
        return;
    }

    let interact_animator = interact_animator.unwrap();
    let player_animator = player_animator.unwrap();
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.anchor = Anchor::BottomCenter;

    let translation = parcel_to_world_location(PLAYER_STARTING_PARCEL);

    //Spawning Entity
    let player = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: player_animator.atlas.clone(),
            transform: Transform {
                scale: Vec3::ONE * PLAYER_SCALE * player_animator.scale,
                translation,
                ..default()
            },
            ..default()
        })
        .insert(player_animator)
        .insert(Name::new("Player"))
        .insert(components::Player {
            speed: PLAYER_SPEED,
            collider_size: PLAYER_COLLIDER,
            current_level: 0,
            current_parcel: Parcel(0, 0),
            level_change_stack: vec![],
        })
        .id();

    let interact_icon = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: interact_animator.atlas.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, ITERACT_ICON_HEIGHT, 0.0)),
            ..default()
        })
        .insert(interact_animator)
        .insert(Name::new("Interact_icon"))
        .insert(components::InteractIcon::default())
        .id();

    let clear_color = ClearColorConfig::Custom(Color::BLACK);
    let mut camera_bundle = Camera2dBundle::new_with_far(10000.0);
    camera_bundle.camera_2d.clear_color = clear_color;
    camera_bundle.transform = Transform::from_translation(Vec3 {
        x: 0.0,
        y: 0.0,
        z: 5000.0,
    });

    camera_bundle.projection.scale = CAMERA_SCALE;
    let camera_entity = commands.spawn(camera_bundle).id();

    commands.entity(player).add_child(camera_entity);
    commands.entity(player).add_child(interact_icon);
}

fn player_movement(
    mut player_query: Query<(
        &mut components::Player,
        &mut Transform,
        &mut components::Animator,
        &mut TextureAtlasSprite,
    )>,
    box_collision_query: Query<(Entity, &GlobalTransform, &components::BoxCollider)>,
    entities_with_level_change: Query<(Entity, &components::LevelChange)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<resources::CollisionMap>,
    time: Res<Time>,
) {
    let result = player_query.get_single_mut();

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return;
    }

    let (mut player, mut transform, mut animator, mut texture_atlas) = result.unwrap();

    let mut animation_state = "Idle";
    let mut movement_input = get_movment_axis_input(&keyboard);

    movement_input = movement_input.normalize();
    movement_input = movement_input * player.speed * PLAYER_SCALE * time.delta_seconds();

    let mut walking = false;
    if movement_input.length() > 0f32 {
        walking = true;
    }

    if walking {
        animation_state = "Run";

        let mut target = transform.translation + Vec3::new(movement_input.x, 0.0, 0.0);

        if !check_player_collision(
            player.as_mut(),
            &target,
            &box_collision_query,
            &entities_with_level_change,
            collision_map.clone(),
        ) {
            transform.translation = target;
        }

        target = transform.translation + Vec3::new(0.0, movement_input.y, movement_input.y * -1.0);

        if !check_player_collision(
            player.as_mut(),
            &target,
            &box_collision_query,
            &entities_with_level_change,
            collision_map.clone(),
        ) {
            transform.translation = target;
        }
    }

    if movement_input.x > 0.0 {
        texture_atlas.flip_x = true;
    } else if movement_input.x < 0.0 {
        texture_atlas.flip_x = false;
    }
    change_animator_state(animator.as_mut(), texture_atlas.as_mut(), animation_state);
}

fn player_interact(
    mut player_query: Query<(&mut components::Player, &mut Transform)>,
    mut iteract_query: Query<(
        &components::InteractIcon,
        &mut components::Animator,
        &mut TextureAtlasSprite,
    )>,
    box_collision_query: Query<(Entity, &GlobalTransform, &components::BoxCollider)>,
    entities_with_level_change: Query<(Entity, &components::LevelChange)>,
    keyboard: Res<Input<KeyCode>>,
    collision_map: Res<resources::CollisionMap>,
) {
    let result = player_query.get_single_mut();

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return;
    }

    let (mut player, mut transform) = result.unwrap();

    let collisions = get_collisions(
        &transform.translation,
        &player.collider_size,
        &box_collision_query,
        &entities_with_level_change,
        collision_map.clone(),
    );

    if keyboard.just_pressed(KeyCode::E) {
        try_change_level(&mut player, &mut transform, &collisions);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        exit_level(&mut player, &mut transform);
    }

    let result = iteract_query.get_single_mut();

    match result {
        Ok((_, mut animator, mut texture_atlas)) => {
            update_interact_icon_visibility(&collisions, animator.as_mut(), texture_atlas.as_mut());
        }

        Err(e) => {
            println!("{}", e);
        }
    }
}

fn update_interact_icon_visibility(
    collisions: &Vec<CollisionResult>,
    interact_icon_animator: &mut components::Animator,
    interact_sprite_atlas: &mut TextureAtlasSprite,
) {
    let mut player_is_in_trigger = false;
    for collision in collisions {
        if collision.collision_type == CollisionType::Trigger {
            player_is_in_trigger = true;
            break;
        }
    }

    if player_is_in_trigger {
        if interact_icon_animator.current_animation.name == "hidden"
            || interact_icon_animator.current_animation.name == "fade_out"
        {
            change_animator_state(interact_icon_animator, interact_sprite_atlas, "fade_in");
            queue_animation(interact_icon_animator, "idle")
        }
    } else if interact_icon_animator.current_animation.name == "idle"
        || interact_icon_animator.current_animation.name == "fade_in"
    {
        change_animator_state(interact_icon_animator, interact_sprite_atlas, "fade_out");
        queue_animation(interact_icon_animator, "hidden")
    }
}
fn change_level(
    player: &mut components::Player,
    player_transform: &mut Transform,
    level_change: &components::LevelChange,
) {
    let level_change_stack_data = LevelChangeStackData {
        level_id: player.current_level,
        location: player_transform.translation,
    };

    player.current_level = level_change.level;
    player.level_change_stack.push(level_change_stack_data);
    player_transform.translation = level_change
        .spawn_point
        .extend(level_change.spawn_point.y * -1f32);

    if level_change.level == 0 {
        player.level_change_stack.clear();
    }
}

fn try_change_level(
    player: &mut components::Player,
    player_transform: &mut Transform,
    collisions: &Vec<CollisionResult>,
) {
    for collision in collisions {
        if collision.collision_type == CollisionType::Trigger && collision.level_change.is_some() {
            let level_change = collision.level_change.clone().unwrap();
            change_level(player, player_transform, &level_change);
        }
    }
}

fn exit_level(player: &mut components::Player, transform: &mut Transform) {
    if let Some(data) = player.level_change_stack.pop() {
        transform.translation = data.location;
        player.current_level = data.level_id;
    }
}

fn check_player_collision(
    player: &mut components::Player,
    target_location: &Vec3,
    box_collision_query: &Query<(Entity, &GlobalTransform, &components::BoxCollider)>,
    entities_with_level_change: &Query<(Entity, &components::LevelChange)>,
    collision_map: resources::CollisionMap,
) -> bool {
    let collisions = get_collisions(
        target_location,
        &player.collider_size,
        box_collision_query,
        entities_with_level_change,
        collision_map,
    );

    for collision in collisions {
        if collision.hit && collision.collision_type == CollisionType::Solid {
            return true;
        }
    }
    false
}

fn get_movment_axis_input(keyboard: &Res<Input<KeyCode>>) -> Vec3 {
    let mut movement_input = Vec3::default();

    if keyboard.pressed(KeyCode::W) {
        movement_input.y += 1f32;
    }

    if keyboard.pressed(KeyCode::S) {
        movement_input.y -= 1f32;
    }

    if keyboard.pressed(KeyCode::D) {
        movement_input.x += 1f32;
    }

    if keyboard.pressed(KeyCode::A) {
        movement_input.x -= 1f32;
    }

    movement_input
}
