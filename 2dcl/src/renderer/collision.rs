use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use dcl2d_ecs_v1::collision_type::CollisionType;

use crate::components::{BoxCollider, LevelChange};

pub const TILE_SIZE: f32 = 1.0;

pub struct CollisionResult {
    pub hit: bool,
    pub collision_type: CollisionType,
    pub level_change: Option<LevelChange>,
}

#[derive(Default, Clone)]
pub struct CollisionMap {
    pub tiles: Vec<CollisionTile>,
    pub tile_size: f32,
}

#[derive(Default, Clone)]
pub struct CollisionTile {
    pub location: Vec2,
    pub colliision_type: CollisionType,
    pub entity: Option<Entity>,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionMap>()
            .add_startup_system_to_stage(StartupStage::PreStartup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(CollisionMap {
        tile_size: TILE_SIZE,
        ..default()
    });
}

pub fn get_mask_collision(
    position: &Vec3,
    size: &Vec2,
    collision_map: CollisionMap,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
) -> CollisionResult {
    for tile in collision_map.tiles {
        let min = Vec2::new(position.x - size.x / 2.0, position.y);
        let max = Vec2::new(position.x + size.x / 2.0, position.y + size.y);
        if tile.location.x > min.x
            && tile.location.x < max.x
            && tile.location.y > min.y
            && tile.location.y < max.y
        {
            if tile.colliision_type == CollisionType::Trigger {
                if let Some(entity) = tile.entity {
                    let level_change =
                        get_level_change_of_entity(entity, entities_with_level_change);
                    return CollisionResult {
                        hit: true,
                        collision_type: tile.colliision_type,
                        level_change,
                    };
                }
            } else {
                return CollisionResult {
                    hit: true,
                    collision_type: tile.colliision_type,
                    level_change: None,
                };
            }
        }
    }

    CollisionResult {
        hit: false,
        collision_type: CollisionType::Solid,
        level_change: None,
    }
}

pub fn get_level_change_of_entity(
    entity: Entity,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
) -> Option<LevelChange> {
    for (current_entity, level_change) in entities_with_level_change {
        if entity == current_entity {
            return Some(level_change.clone());
        }
    }
    None
}

pub fn get_box_collisions(
    position: &Vec3,
    size: &Vec2,
    box_colliders: &Query<(Entity, &GlobalTransform, &BoxCollider)>,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
) -> Vec<CollisionResult> {
    let mut collisions_result: Vec<CollisionResult> = Vec::new();

    for (entity, transform, collider) in box_colliders {
        let collision = collide(
            Vec3 {
                x: position.x,
                y: position.y + size.y / 2.0,
                z: 0.0,
            },
            *size,
            transform.translation() + collider.center.extend(0.0),
            collider.size,
        );

        if collision.is_some() {
            if collider.collision_type == CollisionType::Trigger {
                let level_change = get_level_change_of_entity(entity, entities_with_level_change);
                collisions_result.push(CollisionResult {
                    hit: true,
                    collision_type: collider.collision_type.clone(),
                    level_change,
                });
            } else {
                collisions_result.push(CollisionResult {
                    hit: true,
                    collision_type: collider.collision_type.clone(),
                    level_change: None,
                });
            }
        }
    }

    collisions_result
}

pub fn get_collisions(
    position: &Vec3,
    size: &Vec2,
    box_colliders: &Query<(Entity, &GlobalTransform, &BoxCollider)>,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
    collision_map: CollisionMap,
) -> Vec<CollisionResult> {
    let mut collision_results =
        get_box_collisions(position, size, box_colliders, entities_with_level_change);
    collision_results.push(get_mask_collision(
        position,
        size,
        collision_map,
        entities_with_level_change,
    ));
    collision_results
}
