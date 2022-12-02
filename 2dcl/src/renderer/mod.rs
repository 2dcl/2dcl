use bevy::render::render_resource::SamplerDescriptor;
use bevy::{
    prelude::*,
    render::{render_resource::FilterMode, texture::ImageSettings},
};

pub mod config;
mod custom_material;
mod dcl_3d_scene;
mod error;
mod player_sprite_maker;

mod player;
pub use player::PlayerComponent;
use player::PlayerPlugin;

mod animations;
use animations::AnimationsPlugin;

mod collision;
pub use collision::CollisionMap;
use collision::CollisionPlugin;

pub mod scene_loader;
use scene_loader::SceneLoaderPlugin;

mod scene_maker;
pub use scene_maker::SceneMakerPlugin;

mod scenes_io;
pub use scenes_io::ScenesIOPlugin;

mod debug;
use debug::DebugPlugin;

mod console;
use console::MyConsolePlugin;


pub fn start() {
  
  let current_path = std::env::current_exe().unwrap();
  let current_path = current_path.parent().unwrap();
  std::env::set_current_dir(current_path).unwrap();

  match player_sprite_maker::make_player_spritesheet(
    "./assets/wearables/".to_owned(),
    "./assets/player.json".to_owned(),
  ) {
    Ok(_) => {}
    Err(e) => println!("{}", e),
  };

  let mut app = App::new();
  setup(&mut app);
  app.add_plugin(SceneLoaderPlugin)
  .add_plugin(ScenesIOPlugin)
  .run();
}

pub fn setup(app: &mut bevy::app::App) {
  app.insert_resource(Msaa { samples: 1 })
    .insert_resource(ImageSettings {
      default_sampler: SamplerDescriptor {
        mag_filter: FilterMode::Nearest,
        ..default()
      },
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(SceneMakerPlugin)
    //.add_plugin(DebugPlugin)
    .add_plugin(MyConsolePlugin)
    .add_plugin(AnimationsPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(CollisionPlugin);
}