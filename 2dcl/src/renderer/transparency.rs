use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::{
    constants::{
        PLAYER_VISIBILITY_BOX, PLAYER_VISIBILITY_BOX_OFFSET, TRANSPARENCY_FADE_DURATION_IN_SECONDS,
        TRANSPARENCY_VALUE_FOR_HIDING_ELEMENTS,
    },
    screen_fade,
};
use crate::states::AppState;
use crate::{bundles::get_translation_by_anchor, components};

pub struct TransparencyPlugin;

impl Plugin for TransparencyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_transparency
                .after(screen_fade::update_fade)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                check_elements_overlapping_parcels,
                check_elements_on_top_of_player,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_transparency(
    mut sprites_query: Query<(&mut Sprite, &mut components::SpriteRenderer)>,
    time: Res<Time>,
    fade: Res<screen_fade::Fade>,
) {
    for (mut sprite, mut sprite_renderer) in sprites_query.iter_mut() {
        if sprite_renderer.is_on_top_of_player || sprite_renderer.is_on_top_of_player_parcel {
            sprite_renderer.transparency_alpha +=
                time.delta_seconds() / TRANSPARENCY_FADE_DURATION_IN_SECONDS;
        } else {
            sprite_renderer.transparency_alpha -=
                time.delta_seconds() / TRANSPARENCY_FADE_DURATION_IN_SECONDS;
        }

        sprite_renderer.transparency_alpha = sprite_renderer.transparency_alpha.clamp(0., 1.);

        let new_alpha = TRANSPARENCY_VALUE_FOR_HIDING_ELEMENTS * sprite_renderer.transparency_alpha
            + sprite_renderer.default_color.a() * (1. - sprite_renderer.transparency_alpha);

        let new_alpha = new_alpha * fade.alpha;
        sprite.color.set_a(new_alpha);
    }
}

fn check_elements_on_top_of_player(
    player_query: Query<&GlobalTransform, (Changed<GlobalTransform>, With<components::Player>)>,
    images: Res<Assets<Image>>,
    mut sprites_query: Query<
        (
            &GlobalTransform,
            &Sprite,
            &Handle<Image>,
            &mut components::SpriteRenderer,
        ),
        Without<components::Player>,
    >,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => {
            return;
        }
    };

    for (other_transform, other_sprite, other_image, mut other_sprite_renderer) in
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
        other_sprite_renderer.is_on_top_of_player =
            is_object_covering_player(player_transform.translation(), other_location, other_size);
    }
}

fn is_object_covering_player(
    player_location: Vec3,
    object_location: Vec3,
    object_size: Vec2,
) -> bool {
    if object_location.z < player_location.z {
        return false;
    }

    //Check top left
    if collide(
        Vec3 {
            x: player_location.x + PLAYER_VISIBILITY_BOX_OFFSET.x - PLAYER_VISIBILITY_BOX.x,
            y: player_location.y + PLAYER_VISIBILITY_BOX_OFFSET.y + PLAYER_VISIBILITY_BOX.y,
            z: player_location.z,
        },
        Vec2::ONE,
        object_location,
        object_size,
    )
    .is_none()
    {
        return false;
    }

    //Check top right
    if collide(
        Vec3 {
            x: player_location.x + PLAYER_VISIBILITY_BOX_OFFSET.x + PLAYER_VISIBILITY_BOX.x,
            y: player_location.y + PLAYER_VISIBILITY_BOX_OFFSET.y + PLAYER_VISIBILITY_BOX.y,
            z: player_location.z,
        },
        Vec2::ONE,
        object_location,
        object_size,
    )
    .is_none()
    {
        return false;
    }

    //Check bottom right
    if collide(
        Vec3 {
            x: player_location.x + PLAYER_VISIBILITY_BOX_OFFSET.x + PLAYER_VISIBILITY_BOX.x,
            y: player_location.y + PLAYER_VISIBILITY_BOX_OFFSET.y - PLAYER_VISIBILITY_BOX.y,
            z: player_location.z,
        },
        Vec2::ONE,
        object_location,
        object_size,
    )
    .is_none()
    {
        return false;
    }

    //Check bottom left
    if collide(
        Vec3 {
            x: player_location.x + PLAYER_VISIBILITY_BOX_OFFSET.x - PLAYER_VISIBILITY_BOX.x,
            y: player_location.y + PLAYER_VISIBILITY_BOX_OFFSET.y - PLAYER_VISIBILITY_BOX.y,
            z: player_location.z,
        },
        Vec2::ONE,
        object_location,
        object_size,
    )
    .is_none()
    {
        return false;
    }

    true
}
fn check_elements_overlapping_parcels(
    player_query: Query<&components::Player, Changed<components::Player>>,
    mut sprites_query: Query<&mut components::SpriteRenderer, Without<components::Player>>,
    scenes_query: Query<&components::Scene>,
) {
    let player = match player_query.get_single() {
        Ok(player) => player,
        Err(_) => {
            return;
        }
    };

    for mut sprite_renderer in sprites_query.iter_mut() {
        let sprite_is_in_default_parcel =
            is_sprite_renderer_in_default_parcel(&sprite_renderer, &scenes_query);
        'outer: for parcel_overlapping in sprite_renderer.parcels_overlapping.clone() {
            for scene in scenes_query.iter() {
                if scene.parcels.contains(&parcel_overlapping) {
                    if scene.parcels.contains(&player.current_parcel)
                        && (!sprite_is_in_default_parcel || !scene.is_default)
                    {
                        sprite_renderer.is_on_top_of_player_parcel = true;
                        break 'outer;
                    } else {
                        sprite_renderer.is_on_top_of_player_parcel = false;
                        break;
                    }
                }
            }
        }
    }

    fn is_sprite_renderer_in_default_parcel(
        sprite_renderer: &components::SpriteRenderer,
        scenes_query: &Query<&components::Scene>,
    ) -> bool {
        for scene in scenes_query.iter() {
            for parcel in sprite_renderer.parent_parcels.iter() {
                if scene.parcels.contains(parcel) {
                    return scene.is_default;
                }
            }
        }
        false
    }
}
