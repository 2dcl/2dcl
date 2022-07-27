use bevy::prelude::*;

mod player;
mod debug;
mod scene_deserializer;

use player::PlayerPlugin;
use debug::DebugPlugin;
use scene_deserializer::SceneDeserializerPlugin;



pub fn start() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(SceneDeserializerPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .run();
}




