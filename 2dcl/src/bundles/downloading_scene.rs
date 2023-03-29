use std::path::PathBuf;

use crate::components;
use bevy::{prelude::*, tasks::Task};
use dcl_common::Parcel;

use super::get_parcels_center_location;

pub const MIN_ALPHA: f32 = 0.3;
pub const MAX_ALPHA: f32 = 1.;
pub const ANIMATION_LENGTH_IN_SECONDS: f32 = 1.;

#[derive(Bundle)]
pub struct DownloadingScene {
    pub downloading_scene_component: components::DownloadingScene,
    pub sprite: SpriteBundle,
}
impl DownloadingScene {
    pub fn from_task_and_parcels(
        task: Task<Option<Vec<PathBuf>>>,
        parcels: Vec<Parcel>,
        asset_server: &AssetServer,
    ) -> Self {
        DownloadingScene {
            sprite: SpriteBundle {
                texture: asset_server.load("ui/2dcl_logo.png"),
                transform: Transform {
                    translation: get_parcels_center_location(&parcels),
                    rotation: Quat::IDENTITY,
                    scale: Vec3 {
                        x: 0.25,
                        y: 0.25,
                        z: 0.25,
                    },
                },
                ..default()
            },
            downloading_scene_component: components::DownloadingScene {
                task,
                parcels,
                animation_alpha: 0.,
                animation_forward: true,
            },
        }
    }
}

pub fn downloading_scene_animation(
    mut downloading_scene_query: Query<(
        &mut Sprite,
        &mut components::DownloadingScene,
        &mut Visibility,
    )>,
    player_query: Query<&components::Player>,
    time: Res<Time>,
) {
    let player = match player_query.get_single() {
        Ok(player) => player,
        Err(_) => {
            return;
        }
    };

    for (mut sprite, mut downloading_scene, mut visibility) in downloading_scene_query.iter_mut() {
        if player.current_level != 0 && !downloading_scene.parcels.contains(&player.current_parcel)
        {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;

            if downloading_scene.animation_forward {
                downloading_scene.animation_alpha -=
                    time.delta_seconds() / ANIMATION_LENGTH_IN_SECONDS;
            } else {
                downloading_scene.animation_alpha +=
                    time.delta_seconds() / ANIMATION_LENGTH_IN_SECONDS;
            }

            downloading_scene.animation_alpha = downloading_scene.animation_alpha.clamp(0., 1.);

            if downloading_scene.animation_alpha == 0. {
                downloading_scene.animation_forward = false;
            } else if downloading_scene.animation_alpha == 1. {
                downloading_scene.animation_forward = true;
            }

            let new_alpha = MIN_ALPHA * downloading_scene.animation_alpha
                + MAX_ALPHA * (1. - downloading_scene.animation_alpha);
            sprite.color.set_a(new_alpha);
        }
    }
}
