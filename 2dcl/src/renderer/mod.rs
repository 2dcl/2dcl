use std::{
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use bevy::{asset::ChangeWatcher, log::LogPlugin, prelude::*};

pub mod constants;
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

use crate::resources;

//mod roads_updater;
//use roads_updater::update_roads;

pub fn start() {
    let current_path = std::env::current_exe().unwrap();
    let current_path = current_path.parent().unwrap();

    let mut app = App::new();
    setup(&mut app, "2dcl".to_string(), current_path);
    app.add_plugins((
        SceneLoaderPlugin,
        SceneMakerPlugin,
        ScenesIOPlugin,
        MyConsolePlugin,
    ))
    .run();
}

pub fn setup<P>(app: &mut bevy::app::App, window_title: String, working_dir: P)
where
    P: AsRef<Path>,
{
    let config = resources::Config::from_config_file();
    update_avatar(&config.avatar.eth_address);

    std::env::set_current_dir(&working_dir).unwrap();
    let absolute_base_dir = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
    std::env::set_var("CARGO_MANIFEST_DIR", absolute_base_dir);

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..Default::default()
            })
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
            })
            .disable::<LogPlugin>(),
    )
    .add_plugins(DebugPlugin)
    .add_plugins(ScreenFadePlugin)
    .add_plugins(AnimationsPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(TransparencyPlugin)
    .add_plugins(CollisionPlugin)
    .insert_resource(Msaa::Off)
    .insert_resource(config);
}

pub fn update_avatar(eth_adress: &str) {
    let current_path = std::env::current_exe().unwrap();
    let current_path = current_path.parent().unwrap();
    std::env::set_current_dir(current_path).unwrap();
    let args = vec!["import-avatar".to_string(), eth_adress.to_string()];
    std::process::Command::new(std::env::current_exe().unwrap())
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    match player_sprite_maker::make_player_spritesheet(
        "./assets/wearables/".to_owned(),
        "./assets/player.json".to_owned(),
    ) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}
