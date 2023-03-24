use bevy::prelude::*;

use crate::components;

use super::config::SCREEN_FADE_DURATION_IN_SECONDS;

pub struct FadeFinished(pub FadeDirection);

#[derive(Default, Eq, PartialEq, Clone)]
pub enum FadeDirection {
    #[default]
    FadeIn,
    FadeOut,
}

#[derive(Default, Resource)]
pub struct Fade {
    pub alpha: f32,
    pub direction: FadeDirection,
}

pub struct ScreenFadePlugin;

impl Plugin for ScreenFadePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Fade::default())
            .add_event::<FadeFinished>()
            .add_system(update_fade);
    }
}

pub fn update_fade(
    mut sprite_renderer_query: Query<(&mut Sprite, &mut components::SpriteRenderer)>,
    mut other_sprites_query: Query<&mut Sprite, Without<components::SpriteRenderer>>,
    mut event_fade_finished: EventWriter<FadeFinished>,
    time: Res<Time>,
    mut fade: ResMut<Fade>,
) {
    if fade_finished(&fade) {
        return;
    }

    match &fade.direction {
        FadeDirection::FadeIn => {
            fade.alpha += time.delta_seconds() / SCREEN_FADE_DURATION_IN_SECONDS;
        }
        FadeDirection::FadeOut => {
            fade.alpha -= time.delta_seconds() / SCREEN_FADE_DURATION_IN_SECONDS;
        }
    }

    fade.alpha = fade.alpha.clamp(0., 1.);

    if fade_finished(&fade) {
        event_fade_finished.send(FadeFinished(fade.direction.clone()));
    }

    for (mut sprite, sprite_renderer) in sprite_renderer_query.iter_mut() {
        let color = Color::Rgba {
            red: sprite.color.r(),
            green: sprite.color.g(),
            blue: sprite.color.b(),
            alpha: sprite_renderer.default_color.a() * fade.alpha,
        };
        sprite.color = color;
    }

    for mut sprite in other_sprites_query.iter_mut() {
        let color = Color::Rgba {
            red: sprite.color.r(),
            green: sprite.color.g(),
            blue: sprite.color.b(),
            alpha: fade.alpha,
        };
        sprite.color = color;
    }

    fn fade_finished(fade: &Fade) -> bool {
        (fade.direction == FadeDirection::FadeIn && fade.alpha == 1.)
            || (fade.direction == FadeDirection::FadeOut && fade.alpha == 0.)
    }
}
