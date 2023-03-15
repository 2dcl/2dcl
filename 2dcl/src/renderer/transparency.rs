use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, Anchor},
};

use crate::components;

use super::config::{PLAYER_VISIBILITY_BOX, TRANSPARENCY_VALUE_FOR_ELEMENTS_ON_TOP_OF_PLAYER};

pub fn update_transparency(
    player_query: Query<&GlobalTransform, With<components::Player>>,
    images: Res<Assets<Image>>,
    mut sprites_query: Query<
        (
            &GlobalTransform,
            &mut Sprite,
            &Handle<Image>,
            &components::SpriteRenderer,
        ),
        Without<components::Player>,
    >,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(e) => {
            println!("Player Transform not found in world: {}", e);
            return;
        }
    };

    for (other_transform, mut other_sprite, other_image, other_sprite_renderer) in
        sprites_query.iter_mut()
    {
        let other_size = match images.get(other_image) {
            Some(image) => image.size(),
            None => Vec2::ZERO,
        };

        let other_location = get_translation_by_anchor(
            &other_size,
            &other_transform.translation(),
            &other_sprite.anchor,
        );
        if other_transform.translation().z > player_transform.translation().z
            && collide(
                player_transform.translation(),
                PLAYER_VISIBILITY_BOX,
                other_location,
                other_size,
            )
            .is_some()
        {
            other_sprite
                .color
                .set_a(TRANSPARENCY_VALUE_FOR_ELEMENTS_ON_TOP_OF_PLAYER);
        } else {
            other_sprite
                .color
                .set_a(other_sprite_renderer.default_color.a());
        }
    }
}

fn get_translation_by_anchor(size: &Vec2, translation: &Vec3, anchor: &Anchor) -> Vec3 {
    match anchor {
        Anchor::BottomCenter => Vec3 {
            x: translation.x,
            y: translation.y + size.y / 2.,
            z: translation.z,
        },
        Anchor::BottomLeft => Vec3 {
            x: translation.x + size.x / 2.,
            y: translation.y + size.y / 2.,
            z: translation.z,
        },
        Anchor::BottomRight => Vec3 {
            x: translation.x - size.x / 2.,
            y: translation.y + size.y / 2.,
            z: translation.z,
        },
        Anchor::Center => *translation,
        Anchor::CenterLeft => Vec3 {
            x: translation.x + size.x / 2.,
            y: translation.y,
            z: translation.z,
        },
        Anchor::CenterRight => Vec3 {
            x: translation.x - size.x / 2.,
            y: translation.y,
            z: translation.z,
        },
        Anchor::Custom(vec) => Vec3 {
            x: translation.x - size.x * vec.x,
            y: translation.y - size.y * vec.y,
            z: translation.z,
        },
        Anchor::TopCenter => Vec3 {
            x: translation.x,
            y: translation.y - size.y / 2.,
            z: translation.z,
        },
        Anchor::TopLeft => Vec3 {
            x: translation.x + size.x / 2.,
            y: translation.y - size.y / 2.,
            z: translation.z,
        },
        Anchor::TopRight => Vec3 {
            x: translation.x - size.x / 2.,
            y: translation.y - size.y / 2.,
            z: translation.z,
        },
    }
}
