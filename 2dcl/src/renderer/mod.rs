use bevy::{prelude::*, render::{render_resource::FilterMode, texture::ImageSettings}};
mod player;
mod debug;
mod scene_deserializer;
mod collision;
//mod render_to_texture;
mod animations;
mod player_sprite_maker;
mod psd_reader;

use player::PlayerPlugin;
use animations::AnimationsPlugin;
use debug::DebugPlugin;
use collision::CollisionPlugin;
use scene_deserializer::SceneDeserializerPlugin;
use bevy::render::render_resource::SamplerDescriptor;



pub fn start() {
   // psd_reader::psd_read();
    player_sprite_maker::make_player_spritesheet("./2dcl/assets/wearables/".to_owned(), "./2dcl/assets/player.json".to_owned());

    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ImageSettings{default_sampler: SamplerDescriptor { 
            mag_filter: FilterMode::Nearest,
           ..default()}})
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationsPlugin)
        .add_plugin(SceneDeserializerPlugin)
        .add_plugin(PlayerPlugin)
        //.add_plugin(RenderToTexturePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CollisionPlugin)
        .run();
       
    }




