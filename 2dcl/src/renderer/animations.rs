use bevy::{prelude::*};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use bevy::sprite::Rect;
use bevy::utils::Duration;
use std::path::PathBuf;

#[derive(Component, Debug)]
pub struct Animator
{
    animations: Vec<Animation>,
    pub atlas: Handle<TextureAtlas>,
    pub scale: f32,
    frame_durations: Vec<f32>,
    timer:Timer,
    current_animation: Animation,
}

#[derive(Debug, Clone)]
pub struct Animation
{
    name: String,
    from: usize,
    to: usize,
    direction: Direction,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Direction {
    Forward,
    Reverse,
    PingpongForward,
    PingpongReverse,
} 

 
pub struct AnimationsPlugin;

impl Plugin for  AnimationsPlugin
{

    fn build(&self, app: &mut App) {
    app
       // .add_startup_system_to_stage(StartupStage::PreStartup, load_texture_atlas)
        .add_system(update_animations)
        ;
    }
}

pub fn update_animations
(
    time: Res<Time>,
    mut query: Query<(
        &mut Animator,
        &mut TextureAtlasSprite
    )>,
)
{
    for (mut animator, mut sprite) in &mut query.iter_mut() 
    {
        animator.timer.tick(time.delta());

        if animator.timer.just_finished() 
        {           
            let mut new_index = sprite.index;
            match animator.current_animation.direction
            {
                Direction::Forward =>
                {
                    new_index+= 1;

                    if new_index < animator.current_animation.from || new_index > animator.current_animation.to
                    {
                        new_index = animator.current_animation.from;
                    }
                }

                Direction::Reverse =>
                {
                    if new_index <= animator.current_animation.from || new_index > animator.current_animation.to
                    {   
                        new_index = animator.current_animation.to;
                    }
                    else 
                    {
                        new_index -=1;
                    }
                }

                Direction::PingpongForward =>
                {
                    new_index+= 1;
                    if new_index < animator.current_animation.from || new_index > animator.current_animation.to
                    {
                        animator.current_animation.direction = Direction::PingpongReverse;
                        if animator.current_animation.from<=animator.current_animation.to-1
                        {
                            new_index = animator.current_animation.to-1;
                        }
                        else
                        {
                            new_index-=1;
                        }
                    }
                }

                Direction::PingpongReverse =>
                {
                    if new_index <= animator.current_animation.from || new_index > animator.current_animation.to && animator.current_animation.from +1 <= animator.current_animation.to
                    {   
                        animator.current_animation.direction = Direction::PingpongForward;
                        new_index = animator.current_animation.from+1;    
                        
                    }
                    else 
                    {
                        new_index -=1;
                    }

                }

            }


            sprite.index = new_index;

            let new_duration = Duration::from_secs_f32(animator.frame_durations[sprite.index]);
            animator.timer.set_duration(new_duration);
        }
    }
}

pub fn change_animator_state
(
    mut animator: Mut<Animator>,
    new_state: String
)
{

    if animator.current_animation.name == new_state
    {
        return;
    }

    for  i in 0..animator.animations.len()
    {
        if animator.animations[i].name == new_state
        {   
            animator.current_animation = animator.animations[i].clone();

            let elapsed = animator.timer.duration();
            animator.timer.set_elapsed(elapsed);
            break;
        }
    }
  
    
}


pub fn get_animator <P>(
    path: P,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,

) ->  Result<Animator, String>

where
    P: AsRef<Path>
{

    let file: File;

    match  File::open(&path)
    {
        Ok(v) => {file = v;}
        Err(e) => {return Err(e.to_string());}
    } 

    let spritesheet: aseprite::SpritesheetData;
    
    match serde_json::from_reader(file)
    {
        Ok(v) => {spritesheet = v;}
        Err(e) => {return Err(e.to_string());}
    } 

    let mut image_path = PathBuf::new();
    image_path.push("../..");
    image_path.push(path);
    image_path.pop();
    image_path.push(&spritesheet.meta.image.unwrap_or_default());
 
    let texture = assets.load(image_path);
    let mut animations: Vec<Animation> = Vec::default();
    let mut frame_durations: Vec<f32> = Vec::default();
  

    let mut atlas = TextureAtlas::new_empty
    (
        texture.clone(), 
        Vec2::new(spritesheet.meta.size.w as f32,spritesheet.meta.size.h as f32)
    );

    for frame in spritesheet.frames
    {
        frame_durations.push( frame.duration as f32 / 1000.0);
        let min = Vec2::new(
            frame.frame.x as f32,
            frame.frame.y as f32
        );

        let max = Vec2::new(
            min.x + frame.frame.w as f32,
            min.y + frame.frame.h as f32,
        );

        atlas.add_texture(Rect{min, max});

    }
    

    
    for frame_tag in spritesheet.meta.frame_tags.unwrap_or_default()
    {
        let  direction;
        match frame_tag.direction
        {
            aseprite::Direction::Forward => { direction = Direction::Forward }
            aseprite::Direction::Reverse => { direction = Direction::Reverse }
            aseprite::Direction::Pingpong => { direction = Direction::PingpongForward }
        }

            animations.push(Animation{
            name: frame_tag.name,
            from: frame_tag.from as usize,
            to: frame_tag.to as usize,
            direction,
        });
    }
    
    let scale;
    match  spritesheet.meta.scale.parse::<f32>() 
    {
        Ok(v) => {scale = v;}
        Err(_e) => {scale = 1.0;}
    } 
    
    let current_animation = animations[0].clone();
    let current_duration = frame_durations[0].clone();
    let animator = Animator{
        animations,
        scale: scale,
        atlas: texture_atlases.add(atlas),
        frame_durations,
        timer:Timer::from_seconds(current_duration, true),
        current_animation,
    };

    return Ok(animator);
}

