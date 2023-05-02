use crate::{
    components,
    renderer::{
        constants::{PARCEL_SIZE_X, PARCEL_SIZE_Y},
        scenes_io::SceneData,
    },
};
use bevy::prelude::*;
use dcl_common::Parcel;
use rmp_serde::*;
use serde::Serialize;

#[derive(Bundle, Default)]
pub struct Scene {
    name: Name,
    visibility: VisibilityBundle,
    transform: TransformBundle,
    scene: components::Scene,
}

impl Scene {
    pub fn from_2dcl_scene_data(scene_data: &SceneData) -> Self {
        let location: Vec3 = get_parcels_center_location(&scene_data.parcels);
        let scene = &scene_data.scene;
        let mut scene_u8: Vec<u8> = Vec::new();
        scene
            .serialize(&mut Serializer::new(&mut scene_u8))
            .unwrap();

        Scene {
            name: Name::new(scene.name.clone()),
            transform: TransformBundle::from_transform(Transform::from_translation(location)),
            scene: components::Scene {
                name: scene.name.clone(),
                parcels: scene_data.parcels.clone(),
                timestamp: scene_data.scene.timestamp,
                serialized_data: scene_u8,
                path: scene_data.path.clone(),
                is_default: scene_data.is_default,
            },
            ..default()
        }
    }
}

pub fn get_parcels_center_location(parcels: &Vec<Parcel>) -> Vec3 {
    let mut min: Vec2 = Vec2 {
        x: f32::MAX,
        y: f32::MAX,
    };
    let mut max: Vec2 = Vec2 {
        x: f32::MIN,
        y: f32::MIN,
    };

    for parcel in parcels {
        if (parcel.0 as f32 * PARCEL_SIZE_X) < min.x {
            min.x = parcel.0 as f32 * PARCEL_SIZE_X;
        }

        if (parcel.1 as f32 * PARCEL_SIZE_Y) < min.y {
            min.y = parcel.1 as f32 * PARCEL_SIZE_Y;
        }

        if (parcel.0 as f32 * PARCEL_SIZE_X) > max.x {
            max.x = parcel.0 as f32 * PARCEL_SIZE_X;
        }

        if (parcel.1 as f32 * PARCEL_SIZE_Y) > max.y {
            max.y = parcel.1 as f32 * PARCEL_SIZE_Y;
        }
    }

    Vec3 {
        x: (min.x + max.x) / 2f32,
        y: (min.y + max.y) / 2f32,
        z: (min.y + max.y) / -2f32,
    }
}
