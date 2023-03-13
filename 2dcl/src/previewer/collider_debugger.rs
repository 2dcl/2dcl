use crate::components::BoxCollider;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use dcl2d_ecs_v1::collision_type::CollisionType;

#[derive(Debug, Component)]
pub struct BoxColliderDebug;

pub fn collider_debugger(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<Input<KeyCode>>,
    box_colliders: Query<(Entity, &BoxCollider)>,
    debug_colliders: Query<(Entity, &BoxColliderDebug)>,
) {
    if keyboard.just_pressed(KeyCode::C) {
        for (parent, collider) in box_colliders.iter() {
            let transform = Transform::default()
                .with_translation(Vec3 {
                    x: collider.center.x,
                    y: collider.center.y,
                    z: 100.0,
                })
                .with_scale(Vec3 {
                    x: collider.size.x,
                    y: collider.size.y,
                    z: 1.0,
                });

            let color = match collider.collision_type {
                CollisionType::Trigger => Color::Rgba {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 0.2,
                },
                CollisionType::Solid => Color::GREEN,
            };

            let entity = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform,
                    material: materials.add(ColorMaterial::from(color)),
                    ..default()
                })
                .insert(BoxColliderDebug {})
                .id();

            commands.entity(parent).add_child(entity);
        }
    }

    if keyboard.just_released(KeyCode::C) {
        for (entity, _) in debug_colliders.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
