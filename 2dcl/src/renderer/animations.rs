use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy::utils::Duration;
use serde::Deserialize;
use std::f32::EPSILON;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

#[derive(Component, Debug)]
pub struct Animator {
    pub current_animation: Animation,
    animations: Vec<Animation>,
    pub atlas: Handle<TextureAtlas>,
    pub scale: f32,
    frame_durations: Vec<f32>,
    timer: Timer,
    animation_queue: Vec<Animation>,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    from: usize,
    to: usize,
    direction: Direction,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
    PingpongForward,
    PingpongReverse,
}

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_animations);
    }
}

pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>,
) {
    for (mut animator, mut sprite) in &mut query.iter_mut() {
        animator.timer.tick(time.delta());

        if animator.timer.just_finished() {
            let mut new_index = sprite.index;
            match animator.current_animation.direction {
                Direction::Forward => {
                    new_index += 1;

                    if new_index < animator.current_animation.from
                        || (new_index > animator.current_animation.to
                            && animator.animation_queue.is_empty())
                    {
                        new_index = animator.current_animation.from;
                    } else if new_index > animator.current_animation.to {
                        new_index =
                            play_next_animation_in_queue(animator.as_mut(), sprite.as_mut());
                    }
                }

                Direction::Reverse => {
                    if new_index > animator.current_animation.to
                        || (new_index <= animator.current_animation.from
                            && animator.animation_queue.is_empty())
                    {
                        new_index = animator.current_animation.to;
                    } else if new_index <= animator.current_animation.from {
                        new_index =
                            play_next_animation_in_queue(animator.as_mut(), sprite.as_mut());
                    } else {
                        new_index -= 1;
                    }
                }

                Direction::PingpongForward => {
                    new_index += 1;
                    if new_index < animator.current_animation.from
                        || (new_index > animator.current_animation.to
                            && animator.animation_queue.is_empty())
                    {
                        animator.current_animation.direction = Direction::PingpongReverse;
                        if animator.current_animation.from < animator.current_animation.to {
                            new_index = animator.current_animation.to - 1;
                        } else {
                            new_index -= 1;
                        }
                    } else if new_index > animator.current_animation.to {
                        new_index =
                            play_next_animation_in_queue(animator.as_mut(), sprite.as_mut());
                    }
                }

                Direction::PingpongReverse => {
                    if (new_index > animator.current_animation.to
                        && animator.current_animation.from < animator.current_animation.to)
                        || (new_index <= animator.current_animation.from
                            && animator.animation_queue.is_empty())
                    {
                        animator.current_animation.direction = Direction::PingpongForward;
                        new_index = animator.current_animation.from + 1;
                    } else if new_index <= animator.current_animation.from {
                        new_index =
                            play_next_animation_in_queue(animator.as_mut(), sprite.as_mut());
                    } else {
                        new_index -= 1;
                    }
                }
            }

            change_frame(new_index, sprite.as_mut(), animator.as_mut());
        }
    }
}

fn change_frame(index: usize, sprite: &mut TextureAtlasSprite, animator: &mut Animator) {
    sprite.index = index;
    let new_duration = Duration::from_secs_f32(animator.frame_durations[sprite.index]);
    animator.timer.set_duration(new_duration);
}
pub fn change_animator_state<P>(
    mut animator: &mut Animator,
    sprite: &mut TextureAtlasSprite,
    new_state: P,
) -> Option<Animation>
where
    P: AsRef<str>,
{
    animator.animation_queue.clear();

    if animator.current_animation.name == *new_state.as_ref() {
        return None;
    }

    for i in 0..animator.animations.len() {
        if animator.animations[i].name == *new_state.as_ref() {
            animator.current_animation = animator.animations[i].clone();
            if animator.current_animation.direction == Direction::Forward
                || animator.current_animation.direction == Direction::PingpongForward
            {
                change_frame(animator.current_animation.from, sprite, animator);
            } else {
                change_frame(animator.current_animation.to, sprite, animator);
            }

            return Some(animator.animations[i].clone());
        }
    }

    None
}

pub fn play_next_animation_in_queue(
    animator: &mut Animator,
    sprite: &mut TextureAtlasSprite,
) -> usize {
    let new_state = animator.animation_queue[0].clone().name;
    animator.animation_queue.remove(0);

    if let Some(animation) = change_animator_state(animator, sprite, new_state) {
        return animation.from;
    }

    0
}
pub fn queue_animation<P>(animator: &mut Animator, new_state: P)
where
    P: AsRef<str>,
{
    if animator.current_animation.name == *new_state.as_ref() {
        return;
    }

    for i in 0..animator.animations.len() {
        if animator.animations[i].name == *new_state.as_ref() {
            animator
                .animation_queue
                .push(animator.animations[i].clone());
            break;
        }
    }
}

pub fn get_animator<P>(
    path: P,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Result<Animator, String>
where
    P: AsRef<Path>,
{
    let file: File = match File::open(&path) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let spritesheet: aseprite::SpritesheetData = match serde_json::from_reader(file) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let mut image_path = PathBuf::from(path.as_ref());
    image_path.pop();
    image_path.push(&spritesheet.meta.image.unwrap_or_default());

    let texture = assets.load(image_path);

    let mut animations: Vec<Animation> = Vec::default();
    let mut frame_durations: Vec<f32> = Vec::default();

    let mut atlas = TextureAtlas::new_empty(
        texture,
        Vec2::new(
            spritesheet.meta.size.w as f32,
            spritesheet.meta.size.h as f32,
        ),
    );

    for frame in spritesheet.frames {
        let frame_duration = f32::max(EPSILON, frame.duration as f32 / 1000.0);
        frame_durations.push(frame_duration);
        let min = Vec2::new(frame.frame.x as f32, frame.frame.y as f32);

        let max = Vec2::new(min.x + frame.frame.w as f32, min.y + frame.frame.h as f32);

        atlas.add_texture(Rect { min, max });
    }

    for frame_tag in spritesheet.meta.frame_tags.unwrap_or_default() {
        let direction = match frame_tag.direction {
            aseprite::Direction::Forward => Direction::Forward,
            aseprite::Direction::Reverse => Direction::Reverse,
            aseprite::Direction::Pingpong => Direction::PingpongForward,
        };

        animations.push(Animation {
            name: frame_tag.name,
            from: frame_tag.from as usize,
            to: frame_tag.to as usize,
            direction,
        });
    }

    let scale = match spritesheet.meta.scale.parse::<f32>() {
        Ok(v) => v,
        Err(_e) => 1.0,
    };

    let current_animation = animations[0].clone();
    let current_duration = frame_durations[0];
    let animator = Animator {
        animations,
        scale,
        atlas: texture_atlases.add(atlas),
        frame_durations,
        timer: Timer::from_seconds(current_duration, true),
        current_animation,
        animation_queue: Vec::default(),
    };

    Ok(animator)
}
