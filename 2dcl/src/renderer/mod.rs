use bevy::prelude::*;

pub mod config;
mod custom_material;
mod dcl_3d_scene;
mod error;
mod player_sprite_maker;

pub mod player;
use player::PlayerPlugin;

pub mod animations;
use animations::AnimationsPlugin;

pub mod collision;
use collision::CollisionPlugin;

pub mod scene_loader;
use scene_loader::SceneLoaderPlugin;

mod scene_maker;
pub use scene_maker::SceneMakerPlugin;

pub mod scenes_io;
pub use scenes_io::ScenesIOPlugin;

mod debug;
use debug::DebugPlugin;

mod transparency;
use transparency::TransparencyPlugin;

mod screen_fade;
use screen_fade::ScreenFadePlugin;

use bevy::render::render_resource::{FilterMode, SamplerDescriptor};

mod console;
use console::MyConsolePlugin;

//mod roads_updater;
//use roads_updater::update_roads;


pub fn start() {
    let current_path = std::env::current_exe().unwrap();
    let current_path = current_path.parent().unwrap();
    std::env::set_current_dir(current_path).unwrap();

    update_avatar();

    let mut app = App::new();
    setup(&mut app, "2dcl".to_string());
    app.add_plugin(SceneLoaderPlugin)
        .add_plugin(MyConsolePlugin)
        .add_plugin(SceneMakerPlugin)
        .add_plugin(ScenesIOPlugin)
        .run();
}

pub fn setup(app: &mut bevy::app::App, window_title: String) {
    app.insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: SamplerDescriptor {
                        mag_filter: FilterMode::Nearest,
                        ..default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: window_title,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(DebugPlugin)
        .add_plugin(ScreenFadePlugin)
        .add_plugin(AnimationsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TransparencyPlugin)
        .add_plugin(CollisionPlugin);
}

pub fn update_avatar()
{
  let mut avatar_info_file = std::env::current_exe().unwrap();
  avatar_info_file.pop();
  avatar_info_file.push("avatar_info");
  if avatar_info_file.exists() {
      let eth_address = std::fs::read_to_string(avatar_info_file).unwrap();

      let args = vec![
          "import-avatar".to_string(),
          eth_address.trim().trim_matches('\n').to_string(),
      ];
      std::process::Command::new(std::env::current_exe().unwrap())
          .args(args)
          .spawn()
          .unwrap()
          .wait()
          .unwrap();
  }

  match player_sprite_maker::make_player_spritesheet(
      "./assets/wearables/".to_owned(),
      "./assets/player.json".to_owned(),
  ) {
      Ok(_) => {}
      Err(e) => println!("{}", e),
  };
}