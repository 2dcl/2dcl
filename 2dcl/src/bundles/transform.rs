use bevy::prelude::*;
use dcl_common::Parcel;

use crate::renderer::{scenes_io::SceneData, config::{PARCEL_SIZE_X, PARCEL_SIZE_Y}};

#[derive(Bundle, Default)]
pub struct Transform {
    pub transform: TransformBundle,
}

impl Transform {
    pub fn new(
      transform_component: &dcl2d_ecs_v1::components::Transform,
      scene_data: &SceneData
    ) -> Self {
        let translation = Vec2::new(
            transform_component.location.x as f32,
            transform_component.location.y as f32,
        )
        .extend(-transform_component.location.y as f32);
        let scale = match scene_data.scene.id == 0 && !is_location_in_bounds(translation, &scene_data.parcels)
        {
          true  => todo!("Do no test in levels > 0"),
          false => Vec3::new(transform_component.scale.x, transform_component.scale.y,1.)
        };
        

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


fn is_location_in_bounds(location: Vec3, parcels: &Vec<Parcel>) -> bool
{
    if parcels.len() <= 0 
    {
      return false;
    }

    let mut bounds = get_parcel_rect(&parcels[0]);

  for parcel in parcels {
    let parcel_rect = get_parcel_rect(parcel);
    if parcel_rect.min.x < bounds.min.x 
    {
      bounds.min.x = parcel_rect.min.x;
    }

    if parcel_rect.min.y < bounds.min.y 
    {
      bounds.min.y = parcel_rect.min.y;
    }

    if parcel_rect.max.x > bounds.max.x 
    {
      bounds.max.x = parcel_rect.max.x;
    }

    if parcel_rect.max.y > bounds.max.y 
    {
      bounds.max.y = parcel_rect.max.y;
    }
  }

  let center = Vec3 {
      x: (bounds.min.x + bounds.max.x) / 2.,
      y: (bounds.min.y + bounds.max.y) / 2.,
      z: (bounds.min.y + bounds.max.y) / -2.,
  };


  let target_location = center + location;
  target_location.x < bounds.max.x && target_location.x > bounds.min.x &&
  target_location.y < bounds.max.y && target_location.y > bounds.min.y

}


fn get_parcel_rect(parcel: &Parcel) -> Rect
{
  let parcel_center = Vec2{
    x: parcel.0 as f32 * PARCEL_SIZE_X,
    y: parcel.1 as f32 * PARCEL_SIZE_Y,
  };

  Rect{
    min: Vec2{
        x: parcel_center.x - PARCEL_SIZE_X/2.,
        y: parcel_center.y - PARCEL_SIZE_Y/2.,
    },
    max: Vec2{
      x: parcel_center.x + PARCEL_SIZE_X/2.,
      y: parcel_center.y + PARCEL_SIZE_Y/2.,
  }
  }
}
