use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use dcl_common::Parcel;

use super::player::Player;
use super::scene_loader;
use clap::Parser;
use clap::*;

pub struct MyConsolePlugin;

impl Plugin for MyConsolePlugin{
    fn build(&self, app: &mut App)
    {
        app
        .add_plugin(ConsolePlugin)
        .add_console_command::<TeleportCommand, _, _>(teleport_command);
    }  
}

/// Teleports the player to a specific parcel
#[derive(ConsoleCommand)]
#[console_command(name = "tp")]
struct TeleportCommand {

    parcel_x: i16,
    parcel_y: i16,
}

fn teleport_command(mut tp: ConsoleCommand<TeleportCommand>,
    mut player_query: Query<(&mut Player, &mut Transform)>) 
    {
    let (player, mut transform) = player_query.single_mut();
    if let Some(TeleportCommand { parcel_x, parcel_y }) = tp.take() {
    
        transform.translation = scene_loader::parcel_to_world_location(Parcel(parcel_x,parcel_y));
        reply!(tp, "teleporting to parcel {},{}",parcel_x,parcel_y);

    }
}