use super::renderer::collision;
use bevy::prelude::*;
use ethereum_adapter::EthAddress;
use serde::Deserialize;

//Config defaults
const ETH_ADDRESS: &str = "0x270722b5222968603E4650C3b70A7DfB971Ed5B6";
const AMBIENT_LIGHT: bool = true;
const AMBIENT_LIGHT_BRIGHTNESS: f32 = 0.1;
const AMBIENT_LIGHT_R: f32 = 0.25;
const AMBIENT_LIGHT_G: f32 = 0.25;
const AMBIENT_LIGHT_B: f32 = 0.25;
const STARTING_PARCEL_Y: i16 = 0;
const STARTING_PARCEL_X: i16 = 0;
const MIN_RENDERING_DISTANCE_IN_PARCELS: usize = 4;
const MAX_RENDERING_DISTANCE_IN_PARCELS: usize = 7;
const CAMERA_SIZE: f32 = 1.0;
const PLAYER_SPEED: f32 = 400.0;
const PLAYER_SCALE: f32 = 1.;
const PLAYER_COLLIDER_SIZE_X: f32 = 18.;
const PLAYER_COLLIDER_SIZE_Y: f32 = 20.;

#[derive(Resource, Deserialize, Default, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub avatar: Avatar,
    #[serde(default)]
    pub world: World,
    #[serde(default)]
    pub player: Player,
}

impl Config {
    pub fn from_config_file() -> Self {
        let mut avatar_info_file = std::env::current_exe().unwrap();
        avatar_info_file.pop();
        avatar_info_file.push("config.toml");
        if let Ok(toml_str) = std::fs::read_to_string(avatar_info_file) {
            match toml::from_str::<Config>(&toml_str) {
                Ok(mut toml) => {
                    if toml.player.scale <= 0. {
                        toml.player.scale = PLAYER_SCALE;
                    }

                    if toml.world.camera_size <= 0. {
                        toml.world.camera_size = CAMERA_SIZE;
                    }

                    return toml;
                }
                Err(err) => println!("{}", err),
            }
        } else {
            println!("Missing config.toml file. Loading default values.");
        }

        Config::default()
    }
}

#[derive(Deserialize, PartialEq)]
pub struct Avatar {
    #[serde(skip)]
    pub eth_address: EthAddress,
    #[serde(default = "ambient_light_default")]
    pub ambient_light: bool,
    #[serde(default = "ambient_light_brightness_default")]
    pub ambient_light_brightness: f32,
    #[serde(default = "ambient_light_r_default")]
    pub ambient_light_r: f32,
    #[serde(default = "ambient_light_g_default")]
    pub ambient_light_g: f32,
    #[serde(default = "ambient_light_b_default")]
    pub ambient_light_b: f32,
}

impl Default for Avatar {
    fn default() -> Self {
        Avatar {
            eth_address: eth_address_default(),
            ambient_light: ambient_light_default(),
            ambient_light_brightness: ambient_light_brightness_default(),
            ambient_light_r: ambient_light_r_default(),
            ambient_light_g: ambient_light_g_default(),
            ambient_light_b: ambient_light_b_default(),
        }
    }
}

fn eth_address_default() -> EthAddress {
    EthAddress {
        address: ETH_ADDRESS.to_string(),
    }
}

fn ambient_light_default() -> bool {
    AMBIENT_LIGHT
}

fn ambient_light_brightness_default() -> f32 {
    AMBIENT_LIGHT_BRIGHTNESS
}

fn ambient_light_r_default() -> f32 {
    AMBIENT_LIGHT_R
}
fn ambient_light_g_default() -> f32 {
    AMBIENT_LIGHT_G
}
fn ambient_light_b_default() -> f32 {
    AMBIENT_LIGHT_B
}

#[derive(Deserialize, PartialEq)]
pub struct World {
    #[serde(default = "starting_parcel_x_default")]
    pub starting_parcel_x: i16,
    #[serde(default = "starting_parcel_y_default")]
    pub starting_parcel_y: i16,
    #[serde(default = "min_render_distance_default")]
    pub min_render_distance: usize,
    #[serde(default = "max_render_distance_default")]
    pub max_render_distance: usize,
    #[serde(default = "camera_size_default")]
    pub camera_size: f32,
}

impl Default for World {
    fn default() -> Self {
        World {
            starting_parcel_x: starting_parcel_x_default(),
            starting_parcel_y: starting_parcel_y_default(),
            min_render_distance: min_render_distance_default(),
            max_render_distance: max_render_distance_default(),
            camera_size: camera_size_default(),
        }
    }
}

fn starting_parcel_x_default() -> i16 {
    STARTING_PARCEL_X
}
fn starting_parcel_y_default() -> i16 {
    STARTING_PARCEL_Y
}
fn min_render_distance_default() -> usize {
    MIN_RENDERING_DISTANCE_IN_PARCELS
}
fn max_render_distance_default() -> usize {
    MAX_RENDERING_DISTANCE_IN_PARCELS
}
fn camera_size_default() -> f32 {
    CAMERA_SIZE
}

#[derive(Deserialize, PartialEq)]
pub struct Player {
    #[serde(default = "player_speed_default")]
    pub speed: f32,
    #[serde(default = "player_scale_default")]
    pub scale: f32,
    #[serde(default = "player_collider_size_x_default")]
    pub collider_size_x: f32,
    #[serde(default = "player_collider_size_y_default")]
    pub collider_size_y: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: player_speed_default(),
            scale: player_scale_default(),
            collider_size_x: player_collider_size_x_default(),
            collider_size_y: player_collider_size_y_default(),
        }
    }
}

fn player_speed_default() -> f32 {
    PLAYER_SPEED
}
fn player_scale_default() -> f32 {
    PLAYER_SCALE
}
fn player_collider_size_x_default() -> f32 {
    PLAYER_COLLIDER_SIZE_X
}
fn player_collider_size_y_default() -> f32 {
    PLAYER_COLLIDER_SIZE_Y
}

#[derive(Default, Clone, Resource)]
pub struct CollisionMap {
    pub tiles: Vec<collision::CollisionTile>,
    pub tile_size: f32,
}
