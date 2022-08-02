use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_json::Result;
//use bevy_inspector_egui::Inspectable;

pub struct AnimationsPlugin;


#[derive(Deserialize, Debug)]
struct Animator {
  
    frames: Vec<Frame>,
    meta: Meta
}


#[derive(Deserialize, Debug)]
struct Frame
{   
    startPosition: [i32; 2],
    size:[i32; 2],
    rotated: bool,
    trimmed: bool,
    duration: i32

}

#[derive(Deserialize, Debug)]
struct Meta
{
    image: String,
    format: String,
    size: [i32; 2],
    scale: i32,
    frameTags:Vec<FrameTag>
}

#[derive(Deserialize, Debug)]
struct FrameTag
{
    name: String,
    from: i32,
    to: i32

}


impl Plugin for AnimationsPlugin
{

    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

) {
    if let Ok(file) = File::open("./assets/animation.json")
    {
        let reader = BufReader::new(file);
        let animator: Result<Animator> = serde_json::from_reader(reader);
        if animator.is_ok()
        {
            let animator = animator.unwrap();

            println!("{:?}",animator);
        }
        else
        {
            println!("animator is not ok");
        }
    }
}
