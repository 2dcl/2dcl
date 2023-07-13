use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::components;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new())
                .register_type::<components::Scene>();
        }
    }
}
