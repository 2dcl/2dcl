use super::renderer::collision;
use bevy::prelude::*;
use serde::Deserialize;

//Config defaults
const STARTING_PARCEL_Y: i16 =  0;
const STARTING_PARCEL_X: i16 =  0;
const MIN_RENDERING_DISTANCE_IN_PARCELS: usize = 4;
const MAX_RENDERING_DISTANCE_IN_PARCELS: usize = 7;
const CAMERA_SCALE: f32 = 1.0;
const PLAYER_SPEED: f32 = 1600.0;
const PLAYER_SCALE: f32 = 1.;
const PLAYER_COLLIDER_SIZE_X: f32 = 18.;
const PLAYER_COLLIDER_SIZE_Y: f32 = 20.;
const ETH_ADRESS: &str = "0x5e5d9d1dfd87e9b8b069b8e5d708db92be5ade99";

#[derive(Resource, Deserialize, Default)]
pub struct Config 
{
  #[serde(default)]
  pub avatar: Avatar,
  #[serde(default)]
  pub world: World,
  #[serde(default)]
  pub player: Player,
}

impl Config{

  pub fn from_config_file() -> Self{

    let mut avatar_info_file = std::env::current_exe().unwrap();
    avatar_info_file.pop();
    avatar_info_file.push("config.toml");
    if avatar_info_file.exists() {
        let toml_str = std::fs::read_to_string(avatar_info_file).unwrap();
        return toml::from_str(&toml_str).unwrap();
    } else
    {
      println!("Missing config.toml file. Loading default values.");
    }

    Config::default()
  }
}

#[derive(Deserialize)]
pub struct Avatar 
{
  #[serde(default = "eth_adress_default")]
  pub eth_adress: String
}

impl Default for Avatar{
  fn default() -> Self {
    Avatar { eth_adress: ETH_ADRESS.to_string() } }
}

fn eth_adress_default() -> String{
  ETH_ADRESS.to_string()
}

#[derive(Deserialize)]
pub struct World 
{
  #[serde(default = "starting_parcel_x_default")]
  pub starting_parcel_x: i16,
  #[serde(default = "starting_parcel_y_default")]
  pub starting_parcel_y: i16,
  #[serde(default = "min_render_distance_default")]
  pub min_render_distance: usize,
  #[serde(default = "max_render_distance_default")]
  pub max_render_distance: usize,
  #[serde(default = "camera_scale_default")]
  pub camera_scale: f32,
}

impl Default for World{
  fn default() -> Self {
    World { starting_parcel_x: STARTING_PARCEL_X, starting_parcel_y: STARTING_PARCEL_Y, min_render_distance: MIN_RENDERING_DISTANCE_IN_PARCELS, max_render_distance: MAX_RENDERING_DISTANCE_IN_PARCELS, camera_scale: CAMERA_SCALE }
 }
}

fn starting_parcel_x_default() -> i16{
  STARTING_PARCEL_X
}
fn starting_parcel_y_default() -> i16{
  STARTING_PARCEL_Y
}
fn min_render_distance_default() -> usize{
  MIN_RENDERING_DISTANCE_IN_PARCELS
}
fn max_render_distance_default() -> usize{
  MAX_RENDERING_DISTANCE_IN_PARCELS
}
fn camera_scale_default() -> f32{
  CAMERA_SCALE
}

#[derive(Deserialize)]
pub struct Player 
{
  #[serde(default = "player_speed_default")]
  pub speed: f32,
  #[serde(default = "player_scale_default")]
  pub scale: f32,
  #[serde(default = "player_collider_size_x_default")]
  pub collider_size_x: f32,
  #[serde(default = "player_collider_size_y_default")]
  pub collider_size_y: f32,
}

impl Default for Player{
  fn default() -> Self {
    Player { speed: PLAYER_SPEED, scale: PLAYER_SCALE, collider_size_x: PLAYER_COLLIDER_SIZE_X, collider_size_y: PLAYER_COLLIDER_SIZE_Y }
  }
}

fn player_speed_default() -> f32{
  PLAYER_SPEED
}
fn player_scale_default() -> f32{
  PLAYER_SCALE
}
fn player_collider_size_x_default() -> f32{
  PLAYER_COLLIDER_SIZE_X
}
fn player_collider_size_y_default() -> f32{
  PLAYER_COLLIDER_SIZE_Y
}

#[derive(Default, Clone, Resource)]
pub struct CollisionMap {
  pub tiles: Vec<collision::CollisionTile>,
  pub tile_size: f32,
}