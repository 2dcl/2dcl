use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, Anchor},
};

use crate::{components, bundles::get_translation_by_anchor};

use super::config::{PLAYER_VISIBILITY_BOX, TRANSPARENCY_VALUE_FOR_HIDING_ELEMENTS};

pub fn update_transparency_on_top_of_player(
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
                .set_a(TRANSPARENCY_VALUE_FOR_HIDING_ELEMENTS);
        } else {
            other_sprite
                .color
                .set_a(other_sprite_renderer.default_color.a());
        }
    }
}


pub fn update_overlapping_elements(
  player_query: Query<&components::Player>,
  mut sprites_query: Query<
      (
          &mut Sprite,
          &components::SpriteRenderer,
      ),
      Without<components::Player>,
  >,
  scenes_query: Query<&components::Scene>,
) {
  let player = match player_query.get_single() {
    Ok(player) => player,
    Err(e) => {
        println!("Player not found in world: {}", e);
        return;
    }
  };


  for (mut sprite, sprite_renderer) in sprites_query.iter_mut()
  {
    'outer: for parcel_overlapping in &sprite_renderer.parcels_overlapping
    {
      for scene in scenes_query.iter()
      {
        if scene.parcels.contains(parcel_overlapping)
        {
          if scene.parcels.contains(&player.current_parcel) 
          {
            sprite.color.set_a(TRANSPARENCY_VALUE_FOR_HIDING_ELEMENTS);
          } else
          {
            sprite.color.set_a(sprite_renderer.default_color.a());
          }
          break 'outer;
        }
      }
    }
  }
}