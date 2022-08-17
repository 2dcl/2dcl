use bevy::prelude::*;
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
//use render_to_texture::RenderToTexturePlugin;



pub fn start() {
      psd_reader::psd_read();
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationsPlugin)
        .add_plugin(SceneDeserializerPlugin)
        .add_plugin(PlayerPlugin)
        //.add_plugin(RenderToTexturePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CollisionPlugin)
        .run();
       
    }




