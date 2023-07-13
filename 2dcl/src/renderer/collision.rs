use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use dcl2d_ecs_v1::collision_type::CollisionType;
use dcl_common::Parcel;

use crate::{
    components::{BoxCollider, LevelChange, Scene},
    resources,
};

pub const TILE_SIZE: f32 = 1.0;

pub struct CollisionResult {
    pub hit: bool,
    pub collision_type: CollisionType,
    pub level_change: Option<LevelChange>,
}

#[derive(Default, Clone)]
pub struct CollisionTile {
    pub location: Vec2,
    pub colliision_type: CollisionType,
    pub entity: Option<Entity>,
    pub parcels: Vec<Parcel>,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::CollisionMap>()
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(resources::CollisionMap {
        tile_size: TILE_SIZE,
        ..default()
    });
}

pub fn get_mask_collision(
    current_parcel: &Parcel,
    current_level: usize,
    position: &Vec3,
    size: &Vec2,
    collision_map: &resources::CollisionMap,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
    scenes_query: &Query<&Scene>,
) -> CollisionResult {
    for tile in &collision_map.tiles {
        if !collision_applies_for_current_parcel(
            current_parcel,
            current_level,
            &tile.parcels,
            scenes_query,
        ) {
            continue;
        }
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
                        collision_type: tile.colliision_type.clone(),
                        level_change,
                    };
                }
            } else {
                return CollisionResult {
                    hit: true,
                    collision_type: tile.colliision_type.clone(),
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
    current_parcel: &Parcel,
    current_level: usize,
    position: &Vec3,
    size: &Vec2,
    box_colliders: &Query<(Entity, &GlobalTransform, &BoxCollider)>,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
    scenes_query: &Query<&Scene>,
) -> Vec<CollisionResult> {
    let mut collisions_result: Vec<CollisionResult> = Vec::new();

    for (entity, transform, collider) in box_colliders {
        if !collision_applies_for_current_parcel(
            current_parcel,
            current_level,
            &collider.parcels,
            scenes_query,
        ) {
            continue;
        }

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
    current_parcel: &Parcel,
    current_level: usize,
    position: &Vec3,
    size: &Vec2,
    box_colliders: &Query<(Entity, &GlobalTransform, &BoxCollider)>,
    entities_with_level_change: &Query<(Entity, &LevelChange)>,
    scenes_query: &Query<&Scene>,
    collision_map: &resources::CollisionMap,
) -> Vec<CollisionResult> {
    let mut collision_results = get_box_collisions(
        current_parcel,
        current_level,
        position,
        size,
        box_colliders,
        entities_with_level_change,
        scenes_query,
    );
    collision_results.push(get_mask_collision(
        current_parcel,
        current_level,
        position,
        size,
        collision_map,
        entities_with_level_change,
        scenes_query,
    ));
    collision_results
}

fn collision_applies_for_current_parcel(
    current_parcel: &Parcel,
    current_level: usize,
    collider_parcels: &Vec<Parcel>,
    scenes_query: &Query<&Scene>,
) -> bool {
    if current_level > 0 || collider_parcels.contains(current_parcel) {
        return true;
    }

    let mut current_parcel_is_default = false;
    let mut collider_parcel_is_default = false;
    for scene in scenes_query.iter() {
        if scene.is_default {
            if scene.parcels.contains(current_parcel) {
                current_parcel_is_default = true;
            } else if scene.parcels == *collider_parcels {
                collider_parcel_is_default = true;
            }

            if current_parcel_is_default && collider_parcel_is_default {
                return true;
            }
        }
    }

    false
}
