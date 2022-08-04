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
    scale: i32,
    frame_tags:Vec<FrameTag>
}

#[derive(Deserialize, Debug, Default)]
struct FrameTag
{
    name: String,
    from: i32,
    to: i32

}

#[derive(Default, Component, Debug)]
pub struct Animator
{
    pub animations: Vec<Animation>,
    pub atlas: Handle<TextureAtlas>, 
    pub frame_durations: Vec<f32>,
    pub timer:Timer,
    pub current_animation: Animation,
}

#[derive(Default, Debug, Clone)]
pub struct Animation
{
    pub name: String,
    from: usize,
    to: usize
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
 
            let mut new_index = sprite.index + 1;

            if new_index < animator.current_animation.from || new_index > animator.current_animation.to
            {
                new_index = animator.current_animation.from;
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
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

) -> Animator

{
    let mut animator_data = AnimatorData::default();
    if let Ok(file) = File::open(path)
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
            to: frame_tag.to as usize
        });
    }
    
    let current_animation = animations[0].clone();
    let current_duration = frame_durations[0].clone();
    let animator = Animator{
        animations,
        atlas: texture_atlases.add(atlas),
        frame_durations,
        timer:Timer::from_seconds(current_duration, true),
        current_animation,
    };

    return animator;
}

