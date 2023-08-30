use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;
use dcl_common::Parcel;

use crate::renderer::scene_loader::get_parcel_spawn_point;

use super::player::update_player_scale;
use super::scenes_io::SceneFilesMap;
use super::{content_discovery, update_avatar};
use super::{player::update_camera_size, scene_maker::RoadsData};
use crate::{components, resources};

pub struct MyConsolePlugin;

impl Plugin for MyConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .add_console_command::<TeleportCommand, _>(teleport_command)
            .add_console_command::<ReloadConfig, _>(reload_config)
            .add_console_command::<DiscoverCommand, _>(discover_command)
            .add_console_command::<WhereCommand, _>(where_command);
    }
}

/// Teleports the player to a specific parcel
#[derive(Parser, ConsoleCommand)]
#[command(name = "jump")]
struct TeleportCommand {
    parcel_x: i16,
    parcel_y: i16,
}

/// Prints the current parcel
#[derive(Parser, ConsoleCommand)]
#[command(name = "where")]
struct WhereCommand;

/// Prints the current parcel
#[derive(Parser, ConsoleCommand)]
#[command(name = "discover")]
struct DiscoverCommand;

#[derive(Parser, ConsoleCommand)]
#[command(name = "reload")]
struct ReloadConfig;

fn discover_command(mut discover_cmd: ConsoleCommand<DiscoverCommand>) {
    if discover_cmd.take().is_some() {
        let mut response;
        match content_discovery::find_2d_scenes() {
            Ok(scenes) => {
                response = "Scene\t|\tParcel\t|\tLast Update\n\n".to_string();
                for scene in scenes {
                    let splitted_link: Vec<&str> = scene.link.split('/').collect();

                    let parcel = match splitted_link.len() >= 2 {
                        true => format!(
                            "{} , {}",
                            splitted_link[splitted_link.len() - 2],
                            splitted_link.last().unwrap()
                        ),
                        false => String::default(),
                    };
                    response += &format!("{}\t|\t{}\t|\t{}\n", scene.title, parcel, scene.pub_date);
                }
            }
            Err(err) => {
                response = format!("{}", err);
            }
        }
        reply!(discover_cmd, "{}", response);
    }
}

fn where_command(
    mut where_cmd: ConsoleCommand<WhereCommand>,
    mut player_query: Query<&components::Player>,
) {
    if where_cmd.take().is_some() {
        let player = player_query.single_mut();
        reply!(
            where_cmd,
            "You're in the parcel {},{}",
            player.current_parcel.0,
            player.current_parcel.1
        );
    }
}

fn teleport_command(
    mut tp: ConsoleCommand<TeleportCommand>,
    mut player_query: Query<(&mut components::Player, &mut Transform)>,
    mut roads_data: ResMut<RoadsData>,
    scene_files_map: Res<SceneFilesMap>,
) {
    if let Some(Ok(TeleportCommand { parcel_x, parcel_y })) = tp.take() {
        let (mut player, mut transform) = player_query.single_mut();
        player.current_level = 0;
        transform.translation = get_parcel_spawn_point(
            &Parcel(parcel_x, parcel_y),
            0,
            &mut roads_data,
            &scene_files_map,
        );
        reply!(tp, "teleporting to parcel {},{}", parcel_x, parcel_y);
    }
}

fn reload_config(
    mut reload_command: ConsoleCommand<ReloadConfig>,
    mut commands: Commands,
    config: Res<resources::Config>,
    mut player_query: Query<&mut Transform, With<components::Player>>,
    mut interact_icon_query: Query<
        &mut Transform,
        (With<components::InteractIcon>, Without<components::Player>),
    >,
    mut camera_query: Query<&mut OrthographicProjection>,
) {
    if reload_command.take().is_some() {
        let new_config = resources::Config::from_config_file();

        update_avatar(&new_config.avatar.eth_address);

        if config.player.scale != new_config.player.scale {
            let mut player_transform = player_query.single_mut();
            let mut interact_transform = interact_icon_query.single_mut();
            let mut camera = camera_query.single_mut();

            update_player_scale(
                new_config.player.scale,
                new_config.world.camera_size,
                &mut player_transform,
                &mut interact_transform,
                &mut camera,
            );
        } else if config.world.camera_size != new_config.world.camera_size {
            let mut camera = camera_query.single_mut();
            update_camera_size(
                new_config.world.camera_size,
                new_config.player.scale,
                &mut camera,
            );
        }

        commands.insert_resource(new_config);
    }
}
