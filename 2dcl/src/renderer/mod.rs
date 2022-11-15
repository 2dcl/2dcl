use std::str::FromStr;
use std::path::PathBuf;
use bevy::{prelude::*, render::{render_resource::FilterMode, texture::ImageSettings}};
use bevy::render::render_resource::SamplerDescriptor;

mod dcl_3d_scene;
mod custom_material;
mod player_sprite_maker;
pub mod config;

mod player;
use player::PlayerPlugin;

mod animations;
use animations::AnimationsPlugin;

mod collision;
use collision::CollisionPlugin;

pub mod scene_loader;
use scene_loader::SceneLoaderPlugin;

mod preview;
use preview::PreviewPlugin;

//mod debug;
//use debug::DebugPlugin;

//mod console;
//use console::MyConsolePlugin;

pub fn start() {

  let current_path = std::env::current_exe().unwrap();
  let current_path = current_path.parent().unwrap();
  std::env::set_current_dir(current_path).unwrap();

  player_sprite_maker::make_player_spritesheet("./assets/wearables/".to_owned(), "./2dcl/assets/player.json".to_owned()); 
  let mut app = App::new();
  setup(&mut app);

  app.add_plugin(SceneLoaderPlugin)
      .run();
      
}

pub fn preview_scene(base_dir: std::path::PathBuf)
{
   
    std::env::set_current_dir(&base_dir).unwrap();
    let absolute_base_dir = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
    std::env::set_var("CARGO_MANIFEST_DIR", absolute_base_dir);

    let mut app = App::new();
    setup(&mut app);

    app
        .add_plugin(PreviewPlugin)
        .add_asset::<preview::SceneAsset>()
        .init_asset_loader::<preview::SceneAssetLoader>()
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