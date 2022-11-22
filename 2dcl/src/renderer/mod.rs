use bevy::{prelude::*, render::{render_resource::FilterMode, texture::ImageSettings}};
use bevy::render::render_resource::SamplerDescriptor;

mod dcl_3d_scene;
mod custom_material;
mod player_sprite_maker;
pub mod config;

mod player;
use player::PlayerPlugin;
pub use player::PlayerComponent;

mod animations;
use animations::AnimationsPlugin;

mod collision;
use collision::CollisionPlugin;
pub use collision::CollisionMap;

pub mod scene_loader;
use scene_loader::SceneLoaderPlugin;

//mod debug;
//use debug::DebugPlugin;

//mod console;
//use console::MyConsolePlugin;

pub fn start() {

  let current_path = std::env::current_exe().unwrap();
  let current_path = current_path.parent().unwrap();
  std::env::set_current_dir(current_path).unwrap();

  player_sprite_maker::make_player_spritesheet("./assets/wearables/".to_owned(), "./assets/player.json".to_owned()); 
  let mut app = App::new();
  setup(&mut app);
  app.add_plugin(SceneLoaderPlugin)
      .run();
      
}

pub fn setup(app: &mut bevy::app::App )
{
    app
    .insert_resource(Msaa { samples: 1 })
    .insert_resource(ImageSettings{default_sampler: SamplerDescriptor { 
        mag_filter: FilterMode::Nearest,
      ..default()}})
    //.add_plugin(DebugPlugin)
    //.add_plugin(MyConsolePlugin)
    .add_plugins(DefaultPlugins)
    .add_plugin(AnimationsPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(CollisionPlugin);
} 
