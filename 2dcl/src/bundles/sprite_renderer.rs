use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use bevy::{
    prelude::*,
    sprite::Anchor,
    tasks::{AsyncComputeTaskPool, Task},
};
use imagesize::size;

use crate::renderer::config::LAYERS_DISTANCE;

pub struct LoadingSpriteData {
    pub sprite_renderer_component: dcl2d_ecs_v1::components::SpriteRenderer,
    pub transform: Transform,
    pub texture: Handle<Image>,
    pub image_size: Vec2,
}

#[derive(Bundle, Default)]
pub struct SpriteRenderer {
    pub sprite: SpriteBundle,
}

impl SpriteRenderer {
    pub fn new(
        sprite_renderer_component: &dcl2d_ecs_v1::components::SpriteRenderer,
        transform: &Transform,
        texture: Handle<Image>,
        image_size: Vec2,
    ) -> Self {
      let mut final_transform = transform.clone();

      final_transform.translation = Vec3 {
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z + sprite_renderer_component.layer as f32 * LAYERS_DISTANCE,
        };

        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba {
                    red: (sprite_renderer_component).color.r,
                    green: (sprite_renderer_component).color.g,
                    blue: (sprite_renderer_component).color.b,
                    alpha: (sprite_renderer_component).color.a,
                },

                flip_x: sprite_renderer_component.flip.x,
                flip_y: sprite_renderer_component.flip.y,
                anchor: entity_anchor_to_bevy_anchor(
                    (sprite_renderer_component).anchor.clone(),
                    image_size,
                ),
                ..default()
            },
            transform: final_transform,
            texture,
            ..default()
        };

        SpriteRenderer { sprite }
    }

    pub fn async_load<P>(
        sprite_renderer_component: &dcl2d_ecs_v1::components::SpriteRenderer,
        transform: Transform,
        image_asset_path: P,
        asset_server: &AssetServer,
    ) -> Task<LoadingSpriteData>
    where
        P: AsRef<Path>,
    {
        let image_asset_path = image_asset_path.as_ref().to_path_buf();
        let asset_server_clone = asset_server.clone();
        let sprite_renderer_clone = sprite_renderer_component.clone();
        let mut absolute_path = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
        absolute_path.push(&image_asset_path);

        let image_size = match size(absolute_path) {
            Ok(v) => Vec2::new(v.width as f32, v.height as f32),
            Err(_) => Vec2::new(0.0, 0.0),
        };

        let thread_pool = AsyncComputeTaskPool::get();
        thread_pool.spawn(async move {
            LoadingSpriteData {
                sprite_renderer_component: sprite_renderer_clone.clone(),
                transform,
                texture: asset_server_clone.load(image_asset_path),
                image_size,
            }
        })
    }

    pub fn from_loading_sprite_data(loading_sprite_data: LoadingSpriteData) -> Self {
        Self::new(
            &loading_sprite_data.sprite_renderer_component,
            &loading_sprite_data.transform,
            loading_sprite_data.texture,
            loading_sprite_data.image_size,
        )
    }
}

fn entity_anchor_to_bevy_anchor(anchor: dcl2d_ecs_v1::Anchor, size: Vec2) -> Anchor {
    match anchor {
        dcl2d_ecs_v1::Anchor::BottomCenter => Anchor::BottomCenter,
        dcl2d_ecs_v1::Anchor::BottomLeft => Anchor::BottomLeft,
        dcl2d_ecs_v1::Anchor::BottomRight => Anchor::BottomRight,
        dcl2d_ecs_v1::Anchor::Center => Anchor::Center,
        dcl2d_ecs_v1::Anchor::CenterLeft => Anchor::CenterLeft,
        dcl2d_ecs_v1::Anchor::CenterRight => Anchor::CenterRight,
        dcl2d_ecs_v1::Anchor::Custom(vec) => Anchor::Custom(
            Vec2::new(vec.x as f32 - size.x / 2.0, vec.y as f32 - size.y / 2.0) / size,
        ),
        dcl2d_ecs_v1::Anchor::TopCenter => Anchor::TopCenter,
        dcl2d_ecs_v1::Anchor::TopLeft => Anchor::TopLeft,
        dcl2d_ecs_v1::Anchor::TopRight => Anchor::TopRight,
    }
}
