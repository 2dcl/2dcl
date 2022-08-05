use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_json::Result;
use bevy::sprite::Rect;
use bevy::utils::Duration;


#[derive(Deserialize, Debug, Default)]
struct AnimatorData {
  
    frames: Vec<FrameData>,
    meta: Meta
}


#[derive(Deserialize, Debug, Default)]
struct FrameData
{   
    start_position: [i32; 2],
    size:[i32; 2],
    rotated: bool,
    trimmed: bool,
    duration: i32

}

#[derive(Deserialize, Debug, Default)]
struct Meta
{
    image: String,
    format: String,
    size: [i32; 2],
    scale: f32,
    layer: i32,
    frame_tags:Vec<FrameTag>
}

#[derive(Deserialize, Debug)]
struct FrameTag
{
    name: String,
    from: i32,
    to: i32,
    direction: Direction,

}

#[derive(Component, Debug)]
pub struct Animator
{
    animations: Vec<Animation>,
    pub atlas: Handle<TextureAtlas>, 
    pub layer: i32,
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
    forward,
    backward,
    pingpong,
    pingpong_forward,
    pingpong_backward,
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
                Direction::forward =>
                {
                    new_index+= 1;

                    if new_index < animator.current_animation.from || new_index > animator.current_animation.to
                    {
                        new_index = animator.current_animation.from;
                    }
                }

                Direction::backward =>
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
                Direction::pingpong =>
                {
                    new_index+= 1;
                    animator.current_animation.direction = Direction::pingpong_forward;
                    if new_index < animator.current_animation.from || new_index > animator.current_animation.to
                    {
                        animator.current_animation.direction = Direction::pingpong_backward;
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

                Direction::pingpong_forward =>
                {
                    new_index+= 1;
                    if new_index < animator.current_animation.from || new_index > animator.current_animation.to
                    {
                        animator.current_animation.direction = Direction::pingpong_backward;
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

                Direction::pingpong_backward =>
                {
                    if new_index <= animator.current_animation.from || new_index > animator.current_animation.to && animator.current_animation.from +1 <= animator.current_animation.to
                    {   
                        animator.current_animation.direction = Direction::pingpong_forward;
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


pub fn get_animator(
    path: String,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,

) -> Animator

{
    let mut animator_data = AnimatorData::default();
    if let Ok(file) = File::open(path.clone())
    {
        let reader = BufReader::new(file);
        let result_data: Result<AnimatorData> = serde_json::from_reader(reader);
        if result_data.is_ok()
        {
            animator_data = result_data.unwrap();
        }
        else
        {
            println!("AnimatorData is not ok");
        }
    }
    else
    {   
        println!("Error: file not found");

    }

    let texture = assets.load(&animator_data.meta.image);
    let mut animations: Vec<Animation> = Vec::default();
    let mut frame_durations: Vec<f32> = Vec::default();
  

    let mut atlas = TextureAtlas::new_empty
    (
        texture.clone(), 
        Vec2::new(animator_data.meta.size[0] as f32,animator_data.meta.size[1] as f32)
    );

    for  i in 0..animator_data.frames.len()
    {
        frame_durations.push( animator_data.frames[i].duration as f32 / 1000.0);
        let min = Vec2::new(
            animator_data.frames[i].start_position[0] as f32,
            animator_data.frames[i].start_position[1] as f32
        );

        let max = Vec2::new(
            animator_data.frames[i].size[0] as f32 + min.x,
            animator_data.frames[i].size[1] as f32 + min.y
        );
        atlas.add_texture(Rect{min, max});

    }
    
    for frame_tag in animator_data.meta.frame_tags
    {
            animations.push(Animation{
            name: frame_tag.name,
            from: frame_tag.from as usize,
            to: frame_tag.to as usize,
            direction: frame_tag.direction,
        });
    }
    
    let current_animation = animations[0].clone();
    let current_duration = frame_durations[0].clone();
    let animator = Animator{
        animations,
        scale: animator_data.meta.scale,
        layer: animator_data.meta.layer,
        atlas: texture_atlases.add(atlas),
        frame_durations,
        timer:Timer::from_seconds(current_duration, true),
        current_animation,
    };

    return animator;
}

