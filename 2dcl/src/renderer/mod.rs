use bevy::{prelude::*, render::{render_resource::FilterMode, texture::ImageSettings}};

mod dcl_scene;
mod player;
mod debug;
mod console;
mod collision;
//mod render_to_texture;
mod animations;
mod player_sprite_maker;
pub mod scene_loader;
mod psd_reader;

use player::PlayerPlugin;
use animations::AnimationsPlugin;
use debug::DebugPlugin;
use collision::CollisionPlugin;
use scene_loader::SceneLoaderPlugin;
use bevy::render::render_resource::SamplerDescriptor;
use dcl_common::{Parcel};
use console::MyConsolePlugin;


pub fn start() {

    player_sprite_maker::make_player_spritesheet("./2dcl/assets/wearables/".to_owned(), "./2dcl/assets/player.json".to_owned()); 
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ImageSettings{default_sampler: SamplerDescriptor { 
            mag_filter: FilterMode::Nearest,
           ..default()}})
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationsPlugin)
        .add_plugin(SceneLoaderPlugin)
        .add_plugin(PlayerPlugin)
        //.add_plugin(RenderToTexturePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(MyConsolePlugin)
        .run();
       
}




