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
#[console_command(name = "jump")]
struct TeleportCommand {

    parcel_x: String,
    parcel_y: String,
}

fn teleport_command(mut tp: ConsoleCommand<TeleportCommand>,
    mut player_query: Query<(&mut Player, &mut Transform)>) 
    {
    let (player, mut transform) = player_query.single_mut();
    if let Some(TeleportCommand { parcel_x, parcel_y }) = tp.take() {
    
        if let Ok(parcel_x) = parcel_x.parse::<i16>()
        {
            if let Ok(parcel_y) = parcel_y.parse::<i16>()
            {
                transform.translation = scene_loader::parcel_to_world_location(Parcel(parcel_x,parcel_y));
                reply!(tp, "teleporting to parcel {},{}",parcel_x,parcel_y);
            }
            else
            {
                reply!(tp, "{} is not a valid value",parcel_y);
            }
        }
        else
        {
            reply!(tp, "{} is not a valid value",parcel_x);
        } 

    }
}