use bevy::prelude::*; 
//use bevy_inspector_egui::Inspectable;


pub const TILE_SIZE: f32 = 0.5;


#[derive(Default, Clone)]
pub struct CollisionMap
{
    pub collision_locations: Vec<Vec2>,
    pub tile_size: f32
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App)
    {
        app.init_resource::<CollisionMap>()
        .add_startup_system_to_stage(StartupStage::PreStartup,setup);
    }
}

fn setup(mut commands: Commands,)
{
    commands.insert_resource(CollisionMap{tile_size: TILE_SIZE, ..default()});
}