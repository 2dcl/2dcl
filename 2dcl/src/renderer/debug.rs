
use bevy::prelude::*; 
use bevy_inspector_egui::RegisterInspectable;
use bevy_inspector_egui::WorldInspectorPlugin;

use super::player::Player;

use super::collision::CollisionMap;
use super::scene_loader::CircleCollider;
use super::scene_loader::BoxCollider;
use super::scene_loader::AlphaCollider;

pub struct DebugPlugin;

impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App)
    {
        if cfg!(debug_assertions)
        {   
 
            app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<CollisionMap>()
            .register_inspectable::<CircleCollider>()
            .register_inspectable::<BoxCollider>()
            .register_inspectable::<AlphaCollider>()
            .register_inspectable::<Player>();
        }
    }    
}
