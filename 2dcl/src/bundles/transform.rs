use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Transform {
    pub transform: TransformBundle,
}

impl Transform {
    pub fn from_component(transform_component: &dcl2d_ecs_v1::components::Transform) -> Self {
        let translation = Vec2::new(
            transform_component.location.x as f32,
            transform_component.location.y as f32,
        )
        .extend(-transform_component.location.y as f32);

        let scale = Vec2::new(transform_component.scale.x, transform_component.scale.y).extend(1.);

        let rotation = Quat::from_euler(
            EulerRot::XYZ,
            transform_component.rotation.x.to_radians(),
            transform_component.rotation.y.to_radians(),
            transform_component.rotation.z.to_radians(),
        );

        Transform {
            transform: TransformBundle {
                global: GlobalTransform::default(),
                local: bevy::prelude::Transform {
                    translation,
                    rotation,
                    scale,
                },
            },
        }
    }
}
