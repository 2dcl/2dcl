use bevy::prelude::*;
use serde::Deserialize;
use std::default;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Result;
use bevy::sprite::Rect;
//use bevy_inspector_egui::Inspectable;


#[derive(Deserialize, Debug, Default)]
struct AnimatorData {
  
    frames: Vec<Frame>,
    meta: Meta
}


#[derive(Deserialize, Debug, Default)]
struct Frame
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
    pub current_animation: usize,
    pub timer:Timer,
}

#[derive(Debug)]
pub struct Animation
{
    pub name: String,
    pub atlas: Handle<TextureAtlas>
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
    texture_atlases: Res<Assets<TextureAtlas>>,
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
          
         
            let animation_index = animator.current_animation;
            println!("Updating animation. Current_animation {:?}",animation_index);
            let current_animation = &animator.animations[animation_index.clone()];
            let texture_atlas = texture_atlases.get(&current_animation.atlas).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            // animator.timer.set_duration()
        }
        
    }
}

pub fn change_animator_state
(
    mut animator: Mut<Animator>,
    new_state: String
)
{
    println!("changing state");
           

    for x in 0..animator.animations.len()
    {
        if animator.animations[x].name == new_state
        {    println!("found state, updating ");
            animator.current_animation = x;
            animator.timer.reset();
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
    let mut initial_duration =0.0;

    
    for frame_tag in animator_data.meta.frame_tags
    {
        let mut atlas = TextureAtlas::new_empty
        (
            texture.clone(), 
            Vec2::new(animator_data.meta.size[0] as f32,animator_data.meta.size[1] as f32)
        );

        let mut index = frame_tag.from as usize;
 
        while index <= frame_tag.to as usize && index < animator_data.frames.len()
        {
            if initial_duration == 0.0
            {
                initial_duration = animator_data.frames[index].duration as f32 / 1000.0;
            }
            let min = Vec2::new(
                animator_data.frames[index].start_position[0] as f32,
                animator_data.frames[index].start_position[1] as f32
            );

            let max = Vec2::new(
                animator_data.frames[index].size[0] as f32 + min.x,
                animator_data.frames[index].size[1] as f32 + min.y
            );
            atlas.add_texture(Rect{min, max});
            index+=1;
        }
        println!("Animation len: {:?}",atlas.len());
            animations.push(Animation{
            name: frame_tag.name,
            atlas: texture_atlases.add(atlas)
           
        });
    }

    let animator = Animator{
        animations,
        current_animation: 0,
        timer:Timer::from_seconds(initial_duration, true)
    };

    println!("{:?}",animator);
    return animator;
}

