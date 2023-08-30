use bevy::prelude::*;
use serde::Deserialize;

use crate::{components, states::AppState};

pub type AnimationState = String;
pub struct AnimationPlugin;

#[derive(Deserialize)]
pub struct Animation {
    pub first: usize,
    pub last: usize,
    pub flip_x: bool,
    pub flip_y: bool,
    pub frame_rate: f32,
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite.run_if(in_state(AppState::InGame)));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut components::Animator, &mut TextureAtlasSprite)>,
) {
    for (mut animator, mut sprite) in &mut query {
        animator.timer.tick(time.delta());
        if animator.timer.just_finished() {
            sprite.index += 1;

            let animation_finished = match animator.get_current_animation() {
                Some(current_animation) => animation_finished(sprite.index, current_animation),
                None => false,
            };

            if animation_finished {
                let new_animation = match animator.state_queue.is_empty() {
                    true => animator.current_state.clone(),
                    false => animator.state_queue.remove(0),
                };
                animator.update_state(new_animation);
                sprite.index = animator.get_current_animation().unwrap().first;
            }
        }
    }

    fn animation_finished(current_index: usize, animation: &Animation) -> bool {
        current_index > animation.last || current_index < animation.first
    }
}
/*
#[cfg(test)]
mod test {
    use bevy::prelude::*;
    use std::time::Duration;

    use crate::{bundles, components, renderer::animation::animate_sprite};

    #[test]
    fn sprite_indices_updates_correctly() {
      let mut world = World::default();
      let mut time = Time::default();
      time.update();
      world.insert_resource(time);

      let mut schedule = Schedule::new();
      schedule.add_systems(animate_sprite);

      let entity = world
        .spawn(bundles::Animation {
          sprite_sheet: SpriteSheetBundle::default(),
          indices: components::AnimationIndices { first: 0, last: 1 },
          timer: components::AnimationTimer::new(1.),
        })
        .id();

      let expected_index = 0;
      let actual_index = world.get::<TextureAtlasSprite>(entity).unwrap().index;
      assert_eq!(expected_index, actual_index);

      let mut time = world.resource_mut::<Time>();
      let last_update = time.last_update().unwrap();
      time.update_with_instant(last_update + Duration::from_secs(1));

      // Run system
      schedule.run(&mut world);

      let expected_index = 1;
      let actual_index = world.get::<TextureAtlasSprite>(entity).unwrap().index;
      assert_eq!(expected_index, actual_index);

      let mut time = world.resource_mut::<Time>();
      let last_update = time.last_update().unwrap();
      time.update_with_instant(last_update + Duration::from_secs(1));

      // Run system
      schedule.run(&mut world);

      let expected_index = 0;
      let actual_index = world.get::<TextureAtlasSprite>(entity).unwrap().index;
      assert_eq!(expected_index, actual_index);
    }
}
*/
