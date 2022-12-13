use bevy::prelude::*;
use dcl_common::{Parcel, Result};
use rand::prelude::*;
use rmp_serde::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use super::config::{PARCEL_SIZE_X, PARCEL_SIZE_Y};
use super::scenes_io::SceneData;

const ROADS_DATA_MP_FILE: &str = "./assets/roads/roads.mp";

const DEFAULT_BACKGROUND_PATH: &str = "./background";
const DEFAULT_RANDOM_NO_COLLISION: &str = "./randomized/no_collision";
const DEFAULT_RANDOM_OBSTACLE_SIZE_1: &str = "./randomized/small_collision";
const DEFAULT_RANDOM_OBSTACLE_SIZE_2: &str = "./randomized/big_collision";

const ROAD_BACKGROUND_PATH: &str = "road-background.png";
const LEFT_BORDER_PATH: &str = "road-left.png";
const RIGHT_BORDER_PATH: &str = "road-right.png";
const TOP_BORDER_PATH: &str = "road-top.png";
const BOTTOM_BORDER_PATH: &str = "road-bottom.png";

const CLOSED_TOP_LEFT_CORNER_PATH: &str = "road-closed-top-left.png";
const CLOSED_TOP_RIGHT_CORNER_PATH: &str = "road-closed-top-right.png";
const CLOSED_BOTTOM_LEFT_CORNER_PATH: &str = "road-closed-bottom-left.png";
const CLOSED_BOTTOM_RIGHT_CORNER_PATH: &str = "road-closed-bottom-right.png";

const OPEN_TOP_LEFT_CORNER_PATH: &str = "road-open-top-left.png";
const OPEN_TOP_RIGHT_CORNER_PATH: &str = "road-open-top-right.png";
const OPEN_BOTTOM_LEFT_CORNER_PATH: &str = "road-open-bottom-left.png";
const OPEN_BOTTOM_RIGHT_CORNER_PATH: &str = "road-open-bottom-right.png";

const BOULEVARD_BACKGROUND_PATH: &str = "boulevard-background.png";
const BOULEVARD_TOP_LEFT_PATH: &str = "boulevard-top-left.png";
const BOULEVARD_TOP_RIGHT_PATH: &str = "boulevard-top-right.png";
const BOULEVARD_BOTTOM_LEFT_PATH: &str = "boulevard-bottom-left.png";
const BOULEVARD_BOTTOM_RIGHT_PATH: &str = "boulevard-bottom-right.png";
const BOULEVARD_TOP_PATH: &str = "boulevard-top.png";
const BOULEVARD_RIGHT_PATH: &str = "boulevard-right.png";
const BOULEVARD_LEFT_PATH: &str = "boulevard-left.png";
const BOULEVARD_BOTTOM_PATH: &str = "boulevard-bottom.png";

const TILE_SIZE: (i32, i32) = (64, 64);

const BOULEVARD_SMALL_SIDE_SIZE: usize = 1;
const BOULEVARD_LONG_SIDE_SIZE: usize = 3;
const BOULEVARD_SPACING: i16 = 4;

const RANDOM_OBSTACLE_SPAWN_CHANCE: f64 = 0.1;

#[derive(Debug, Clone, Default)]
pub struct RoadsData {
    pub parcel_map: HashMap<(i16, i16), ()>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SerializableRoadsData {
    parcels: Vec<Parcel>,
}

enum Border {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum CornerType {
    #[default]
    None,
    Closed,
    Open,
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
enum CornerPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone)]
enum BoulevardType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct SceneMakerPlugin;

impl Plugin for SceneMakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands) {
    match read_roads_data() {
        Ok(roads_data) => commands.insert_resource(roads_data),

        Err(e) => println!("error:{}", e),
    }
}

pub fn make_default_scene(parcel: &Parcel) -> Result<SceneData> {
    let mut scene_data = SceneData::default();
    scene_data.parcels.push(parcel.clone());
    scene_data.scene.name = "default_scene".to_string();
    let mut entities: Vec<dcl2d_ecs_v1::Entity> = Vec::new();
    entities.append(&mut make_default_background_entities());
    entities.append(&mut make_default_random_entities());

    let level = dcl2d_ecs_v1::Level {
        entities,
        ..Default::default()
    };
    scene_data.scene.levels.push(level);

    scene_data.path.push("..");
    scene_data.path.push("assets");
    scene_data.path.push("default_scene");

    Ok(scene_data)
}
pub fn make_road_scene(roads_data: &RoadsData, parcel: &Parcel) -> Result<SceneData> {
    let mut entities: Vec<dcl2d_ecs_v1::Entity> = Vec::new();
    let mut corner_entities = make_corners(roads_data, parcel);
    entities.append(&mut make_road_background_entities());
    entities.append(&mut make_boulevard_entities(roads_data, parcel));
    entities.append(&mut make_sidewalk_entities(roads_data, parcel));
    entities.append(&mut corner_entities);

    let level = dcl2d_ecs_v1::Level {
        name: format!("Road {} - {}", &parcel.0, &parcel.1),
        entities,
        ..default()
    };

    let mut path = PathBuf::new();
    path.push("..");
    path.push("assets");
    path.push("roads");

    let scene = dcl2d_ecs_v1::Scene {
        name: format!("Road {} - {}", &parcel.0, &parcel.1),
        levels: vec![level],
        ..default()
    };

    let scene_data = SceneData {
        scene,
        parcels: vec![parcel.clone()],
        path,
    };

    Ok(scene_data)
}

fn make_boulevard_entities(roads_data: &RoadsData, parcel: &Parcel) -> Vec<dcl2d_ecs_v1::Entity> {
    let mut boulevard = Vec::new();

    if (parcel.0 % BOULEVARD_SPACING == 1
        || parcel.0 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1)
        || parcel.1 % BOULEVARD_SPACING == 0)
        && is_road(&Parcel(parcel.0 - 1, parcel.1), roads_data)
        && is_road(&Parcel(parcel.0, parcel.1 + 1), roads_data)
        && is_road(&Parcel(parcel.0 - 1, parcel.1 + 1), roads_data)
    {
        let mut make_boulevard = false;
        let mut size: dcl2d_ecs_v1::Vec2<usize> = dcl2d_ecs_v1::Vec2 {
            x: BOULEVARD_SMALL_SIDE_SIZE,
            y: BOULEVARD_SMALL_SIDE_SIZE,
        };

        if (parcel.0 % BOULEVARD_SPACING == 1
            || parcel.0 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1))
            && is_road(&Parcel(parcel.0 - 2, parcel.1), roads_data)
            && is_road(&Parcel(parcel.0 - 2, parcel.1 + 1), roads_data)
        {
            size.x = BOULEVARD_LONG_SIDE_SIZE;
            make_boulevard = true;
        }

        if parcel.1 % BOULEVARD_SPACING == 0
            && is_road(&Parcel(parcel.0, parcel.1 + 2), roads_data)
            && is_road(&Parcel(parcel.0 - 1, parcel.1 + 2), roads_data)
        {
            size.y = BOULEVARD_LONG_SIDE_SIZE;
            make_boulevard = true;
        }

        if make_boulevard {
            boulevard.append(&mut make_boulevard_entity(BoulevardType::TopLeft, size));
        }
    }

    if (parcel.0 % BOULEVARD_SPACING == 0 || parcel.1 % BOULEVARD_SPACING == 0)
        && is_road(&Parcel(parcel.0 + 1, parcel.1), roads_data)
        && is_road(&Parcel(parcel.0, parcel.1 + 1), roads_data)
        && is_road(&Parcel(parcel.0 + 1, parcel.1 + 1), roads_data)
    {
        let mut make_boulevard = false;

        let mut size: dcl2d_ecs_v1::Vec2<usize> = dcl2d_ecs_v1::Vec2 {
            x: BOULEVARD_SMALL_SIDE_SIZE,
            y: BOULEVARD_SMALL_SIDE_SIZE,
        };

        if parcel.0 % BOULEVARD_SPACING == 0
            && is_road(&Parcel(parcel.0 + 2, parcel.1), roads_data)
            && is_road(&Parcel(parcel.0 + 2, parcel.1 + 1), roads_data)
        {
            make_boulevard = true;
            size.x = BOULEVARD_LONG_SIDE_SIZE;
        }

        if parcel.1 % BOULEVARD_SPACING == 0
            && is_road(&Parcel(parcel.0, parcel.1 + 2), roads_data)
            && is_road(&Parcel(parcel.0 + 1, parcel.1 + 2), roads_data)
        {
            make_boulevard = true;
            size.y = BOULEVARD_LONG_SIDE_SIZE;
        }
        if make_boulevard {
            boulevard.append(&mut make_boulevard_entity(BoulevardType::TopRight, size));
        }
    }

    if (parcel.0 % BOULEVARD_SPACING == 1
        || parcel.0 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1)
        || parcel.1 % BOULEVARD_SPACING == 1
        || parcel.1 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1))
        && is_road(&Parcel(parcel.0 - 1, parcel.1), roads_data)
        && is_road(&Parcel(parcel.0, parcel.1 - 1), roads_data)
        && is_road(&Parcel(parcel.0 - 1, parcel.1 - 1), roads_data)
    {
        let mut make_boulevard = false;
        let mut size: dcl2d_ecs_v1::Vec2<usize> = dcl2d_ecs_v1::Vec2 {
            x: BOULEVARD_SMALL_SIDE_SIZE,
            y: BOULEVARD_SMALL_SIDE_SIZE,
        };

        if (parcel.0 % BOULEVARD_SPACING == 1
            || parcel.0 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1))
            && is_road(&Parcel(parcel.0 - 2, parcel.1), roads_data)
            && is_road(&Parcel(parcel.0 - 2, parcel.1 - 1), roads_data)
        {
            make_boulevard = true;
            size.x = BOULEVARD_LONG_SIDE_SIZE;
        }

        if (parcel.1 % BOULEVARD_SPACING == 1
            || parcel.1 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1))
            && is_road(&Parcel(parcel.0, parcel.1 - 2), roads_data)
            && is_road(&Parcel(parcel.0 - 1, parcel.1 - 2), roads_data)
        {
            make_boulevard = true;
            size.y = BOULEVARD_LONG_SIDE_SIZE;
        }

        if make_boulevard {
            boulevard.append(&mut make_boulevard_entity(BoulevardType::BottomLeft, size));
        }
    }

    if (parcel.0 % BOULEVARD_SPACING == 0
        && is_road(&Parcel(parcel.0 + 2, parcel.1), roads_data)
        && is_road(&Parcel(parcel.0 + 2, parcel.1 - 1), roads_data))
        || ((parcel.1 % BOULEVARD_SPACING == 1
            || parcel.1 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1))
            && is_road(&Parcel(parcel.0, parcel.1 - 2), roads_data)
            && is_road(&Parcel(parcel.0 + 1, parcel.1 - 2), roads_data))
    {
        let mut make_boulevard = false;
        let mut size: dcl2d_ecs_v1::Vec2<usize> = dcl2d_ecs_v1::Vec2 {
            x: BOULEVARD_SMALL_SIDE_SIZE,
            y: BOULEVARD_SMALL_SIDE_SIZE,
        };

        if parcel.0 % BOULEVARD_SPACING == 0
            && is_road(&Parcel(parcel.0 + 2, parcel.1), roads_data)
            && is_road(&Parcel(parcel.0 + 2, parcel.1 - 1), roads_data)
        {
            make_boulevard = true;
            size.x = BOULEVARD_LONG_SIDE_SIZE;
        }

        if (parcel.1 % BOULEVARD_SPACING == 1
            || parcel.1 % BOULEVARD_SPACING == -(BOULEVARD_SPACING - 1))
            && is_road(&Parcel(parcel.0, parcel.1 - 2), roads_data)
            && is_road(&Parcel(parcel.0 + 1, parcel.1 - 2), roads_data)
        {
            make_boulevard = true;
            size.y = BOULEVARD_LONG_SIDE_SIZE;
        }
        if make_boulevard {
            boulevard.append(&mut make_boulevard_entity(BoulevardType::BottomRight, size));
        }
    }

    boulevard
}

fn make_boulevard_entity(
    boulevard_type: BoulevardType,
    size: dcl2d_ecs_v1::Vec2<usize>,
) -> Vec<dcl2d_ecs_v1::Entity> {
    let mut boulevard = Vec::new();

    match boulevard_type {
        BoulevardType::TopLeft => {
            for x in 0..size.x {
                for y in 0..size.y {
                    let transform = dcl2d_ecs_v1::components::Transform {
                        location: dcl2d_ecs_v1::Vec2 {
                            x: PARCEL_SIZE_X as i32 / -2 + x as i32 * TILE_SIZE.0 * 2,
                            y: PARCEL_SIZE_Y as i32 / 2 - y as i32 * TILE_SIZE.0 * 2,
                        },
                        rotation: dcl2d_ecs_v1::Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                    };

                    let sprite = {
                        if x == size.x - 1 && y == size.y - 1 {
                            BOULEVARD_BOTTOM_RIGHT_PATH.to_string()
                        } else if x == size.x - 1 {
                            BOULEVARD_RIGHT_PATH.to_string()
                        } else if y == size.y - 1 {
                            BOULEVARD_BOTTOM_PATH.to_string()
                        } else {
                            BOULEVARD_BACKGROUND_PATH.to_string()
                        }
                    };

                    let anchor = dcl2d_ecs_v1::Anchor::TopLeft;

                    let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                        sprite,
                        layer: -2,
                        anchor,
                        ..default()
                    };

                    let entity = dcl2d_ecs_v1::Entity {
                        name: "Boulevard".to_string(),
                        components: vec![Box::new(renderer), Box::new(transform)],
                        ..default()
                    };
                    boulevard.push(entity);
                }
            }
        }
        BoulevardType::TopRight => {
            for x in 0..size.x {
                for y in 0..size.y {
                    let transform = dcl2d_ecs_v1::components::Transform {
                        location: dcl2d_ecs_v1::Vec2 {
                            x: PARCEL_SIZE_X as i32 / 2 - x as i32 * TILE_SIZE.0 * 2,
                            y: PARCEL_SIZE_Y as i32 / 2 - y as i32 * TILE_SIZE.0 * 2,
                        },
                        rotation: dcl2d_ecs_v1::Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                    };

                    let sprite = {
                        if x == size.x - 1 && y == size.y - 1 {
                            BOULEVARD_BOTTOM_LEFT_PATH.to_string()
                        } else if x == size.x - 1 {
                            BOULEVARD_LEFT_PATH.to_string()
                        } else if y == size.y - 1 {
                            BOULEVARD_BOTTOM_PATH.to_string()
                        } else {
                            BOULEVARD_BACKGROUND_PATH.to_string()
                        }
                    };

                    let anchor = dcl2d_ecs_v1::Anchor::TopRight;

                    let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                        sprite,
                        layer: -2,
                        anchor,
                        ..default()
                    };

                    let entity = dcl2d_ecs_v1::Entity {
                        name: "Boulevard".to_string(),
                        components: vec![Box::new(renderer), Box::new(transform)],
                        ..default()
                    };
                    boulevard.push(entity);
                }
            }
        }
        BoulevardType::BottomLeft => {
            for x in 0..size.x {
                for y in 0..size.y {
                    let transform = dcl2d_ecs_v1::components::Transform {
                        location: dcl2d_ecs_v1::Vec2 {
                            x: PARCEL_SIZE_X as i32 / -2 + x as i32 * TILE_SIZE.0 * 2,
                            y: PARCEL_SIZE_Y as i32 / -2 + y as i32 * TILE_SIZE.0 * 2,
                        },
                        rotation: dcl2d_ecs_v1::Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                    };

                    let sprite = {
                        if x == size.x - 1 && y == size.y - 1 {
                            BOULEVARD_TOP_RIGHT_PATH.to_string()
                        } else if x == size.x - 1 {
                            BOULEVARD_RIGHT_PATH.to_string()
                        } else if y == size.y - 1 {
                            BOULEVARD_TOP_PATH.to_string()
                        } else {
                            BOULEVARD_BACKGROUND_PATH.to_string()
                        }
                    };

                    let anchor = dcl2d_ecs_v1::Anchor::BottomLeft;

                    let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                        sprite,
                        layer: -2,
                        anchor,
                        ..default()
                    };

                    let entity = dcl2d_ecs_v1::Entity {
                        name: "Boulevard".to_string(),
                        components: vec![Box::new(renderer), Box::new(transform)],
                        ..default()
                    };
                    boulevard.push(entity);
                }
            }
        }
        BoulevardType::BottomRight => {
            for x in 0..size.x {
                for y in 0..size.y {
                    let transform = dcl2d_ecs_v1::components::Transform {
                        location: dcl2d_ecs_v1::Vec2 {
                            x: PARCEL_SIZE_X as i32 / 2 - x as i32 * TILE_SIZE.0 * 2,
                            y: PARCEL_SIZE_Y as i32 / -2 + y as i32 * TILE_SIZE.0 * 2,
                        },
                        rotation: dcl2d_ecs_v1::Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                    };

                    let sprite = {
                        if x == size.x - 1 && y == size.y - 1 {
                            BOULEVARD_TOP_LEFT_PATH.to_string()
                        } else if x == size.x - 1 {
                            BOULEVARD_LEFT_PATH.to_string()
                        } else if y == size.y - 1 {
                            BOULEVARD_TOP_PATH.to_string()
                        } else {
                            BOULEVARD_BACKGROUND_PATH.to_string()
                        }
                    };

                    let anchor = dcl2d_ecs_v1::Anchor::BottomRight;

                    let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                        sprite,
                        layer: -2,
                        anchor,
                        ..default()
                    };

                    let entity = dcl2d_ecs_v1::Entity {
                        name: "Boulevard".to_string(),
                        components: vec![Box::new(renderer), Box::new(transform)],
                        ..default()
                    };
                    boulevard.push(entity);
                }
            }
        }
    }

    boulevard
}

fn make_sidewalk_entities(roads_data: &RoadsData, parcel: &Parcel) -> Vec<dcl2d_ecs_v1::Entity> {
    let mut border = Vec::new();
    if !is_road(&Parcel(parcel.0 - 1, parcel.1), roads_data) {
        border.append(&mut make_sidewalk_entity(&Border::Left));
    }

    if !is_road(&Parcel(parcel.0 + 1, parcel.1), roads_data) {
        border.append(&mut make_sidewalk_entity(&Border::Right))
    }

    if !is_road(&Parcel(parcel.0, parcel.1 - 1), roads_data) {
        border.append(&mut make_sidewalk_entity(&Border::Bottom))
    }

    if !is_road(&Parcel(parcel.0, parcel.1 + 1), roads_data) {
        border.append(&mut make_sidewalk_entity(&Border::Top))
    }

    border
}

fn make_sidewalk_entity(border: &Border) -> Vec<dcl2d_ecs_v1::Entity> {
    let mut entities = Vec::new();

    let total_tiles = match border {
        Border::Left => PARCEL_SIZE_Y as i32 / (TILE_SIZE.1 * 2),
        Border::Right => PARCEL_SIZE_Y as i32 / (TILE_SIZE.1 * 2),
        Border::Top => PARCEL_SIZE_X as i32 / (TILE_SIZE.0 * 2),
        Border::Bottom => PARCEL_SIZE_X as i32 / (TILE_SIZE.0 * 2),
    };

    for i in 1..total_tiles - 1 {
        let location_x = match border {
            Border::Left => PARCEL_SIZE_X as i32 / -2,
            Border::Right => PARCEL_SIZE_X as i32 / 2,
            Border::Top => TILE_SIZE.0 * 2 * i - PARCEL_SIZE_X as i32 / 2,
            Border::Bottom => TILE_SIZE.0 * 2 * i - PARCEL_SIZE_X as i32 / 2,
        };

        let location_y = match border {
            Border::Left => TILE_SIZE.1 * 2 * i - PARCEL_SIZE_Y as i32 / 2,
            Border::Right => TILE_SIZE.1 * 2 * i - PARCEL_SIZE_Y as i32 / 2,
            Border::Top => PARCEL_SIZE_Y as i32 / 2,
            Border::Bottom => PARCEL_SIZE_Y as i32 / -2,
        };

        let transform = dcl2d_ecs_v1::components::Transform {
            location: dcl2d_ecs_v1::Vec2 {
                x: location_x,
                y: location_y,
            },
            rotation: dcl2d_ecs_v1::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
        };

        let sprite = match border {
            Border::Left => LEFT_BORDER_PATH.to_string(),
            Border::Right => RIGHT_BORDER_PATH.to_string(),
            Border::Top => TOP_BORDER_PATH.to_string(),
            Border::Bottom => BOTTOM_BORDER_PATH.to_string(),
        };

        let anchor = match border {
            Border::Left => dcl2d_ecs_v1::Anchor::BottomLeft,
            Border::Right => dcl2d_ecs_v1::Anchor::BottomRight,
            Border::Top => dcl2d_ecs_v1::Anchor::TopLeft,
            Border::Bottom => dcl2d_ecs_v1::Anchor::BottomLeft,
        };

        let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
            sprite,
            layer: -2,
            anchor,
            ..default()
        };

        let entity = dcl2d_ecs_v1::Entity {
            name: "Sidewalk".to_string(),
            components: vec![Box::new(renderer), Box::new(transform)],
            ..default()
        };

        entities.push(entity);
    }

    entities
}

fn make_corners(roads_data: &RoadsData, parcel: &Parcel) -> Vec<dcl2d_ecs_v1::Entity> {
    let mut corner_entities = Vec::new();

    let top_left_corner = get_corner_type(roads_data, parcel, CornerPosition::TopLeft);
    match make_corner_entity(&top_left_corner, &CornerPosition::TopLeft) {
        Some(v) => corner_entities.push(v),
        None => {}
    }

    let top_right_corner = get_corner_type(roads_data, parcel, CornerPosition::TopRight);
    match make_corner_entity(&top_right_corner, &CornerPosition::TopRight) {
        Some(v) => corner_entities.push(v),
        None => {}
    }

    let bottom_left_corner = get_corner_type(roads_data, parcel, CornerPosition::BottomLeft);
    match make_corner_entity(&bottom_left_corner, &CornerPosition::BottomLeft) {
        Some(v) => corner_entities.push(v),
        None => {}
    }

    let bottom_right_corner = get_corner_type(roads_data, parcel, CornerPosition::BottomRight);
    match make_corner_entity(&bottom_right_corner, &CornerPosition::BottomRight) {
        Some(v) => corner_entities.push(v),
        None => {}
    }

    corner_entities
}

fn get_corner_type(
    roads_data: &RoadsData,
    parcel: &Parcel,
    position: CornerPosition,
) -> CornerType {
    let mut corner_type = CornerType::default();

    let x_neighbor_is_road;
    let y_neighbor_is_road;
    let diagonal_is_road;

    match position {
        CornerPosition::TopLeft => {
            x_neighbor_is_road = is_road(&Parcel(parcel.0 - 1, parcel.1), roads_data);
            y_neighbor_is_road = is_road(&Parcel(parcel.0, parcel.1 + 1), roads_data);
            diagonal_is_road = is_road(&Parcel(parcel.0 - 1, parcel.1 + 1), roads_data);
        }
        CornerPosition::TopRight => {
            x_neighbor_is_road = is_road(&Parcel(parcel.0 + 1, parcel.1), roads_data);
            y_neighbor_is_road = is_road(&Parcel(parcel.0, parcel.1 + 1), roads_data);
            diagonal_is_road = is_road(&Parcel(parcel.0 + 1, parcel.1 + 1), roads_data);
        }
        CornerPosition::BottomLeft => {
            x_neighbor_is_road = is_road(&Parcel(parcel.0 - 1, parcel.1), roads_data);
            y_neighbor_is_road = is_road(&Parcel(parcel.0, parcel.1 - 1), roads_data);
            diagonal_is_road = is_road(&Parcel(parcel.0 - 1, parcel.1 - 1), roads_data);
        }
        CornerPosition::BottomRight => {
            x_neighbor_is_road = is_road(&Parcel(parcel.0 + 1, parcel.1), roads_data);
            y_neighbor_is_road = is_road(&Parcel(parcel.0, parcel.1 - 1), roads_data);
            diagonal_is_road = is_road(&Parcel(parcel.0 + 1, parcel.1 - 1), roads_data);
        }
    }

    if !x_neighbor_is_road && !y_neighbor_is_road {
        corner_type = CornerType::Closed;
    } else if x_neighbor_is_road && y_neighbor_is_road && !diagonal_is_road {
        corner_type = CornerType::Open;
    } else if x_neighbor_is_road && !y_neighbor_is_road {
        corner_type = CornerType::Horizontal;
    } else if !x_neighbor_is_road && y_neighbor_is_road {
        corner_type = CornerType::Vertical;
    }

    corner_type
}
fn make_corner_entity(
    corner_type: &CornerType,
    position: &CornerPosition,
) -> Option<dcl2d_ecs_v1::Entity> {
    let sprite = match position {
        CornerPosition::TopLeft => match corner_type {
            CornerType::None => return None,
            CornerType::Closed => CLOSED_TOP_LEFT_CORNER_PATH.to_string(),
            CornerType::Open => OPEN_TOP_LEFT_CORNER_PATH.to_string(),
            CornerType::Vertical => LEFT_BORDER_PATH.to_string(),
            CornerType::Horizontal => TOP_BORDER_PATH.to_string(),
        },
        CornerPosition::TopRight => match corner_type {
            CornerType::None => return None,
            CornerType::Closed => CLOSED_TOP_RIGHT_CORNER_PATH.to_string(),
            CornerType::Open => OPEN_TOP_RIGHT_CORNER_PATH.to_string(),
            CornerType::Vertical => RIGHT_BORDER_PATH.to_string(),
            CornerType::Horizontal => TOP_BORDER_PATH.to_string(),
        },
        CornerPosition::BottomRight => match corner_type {
            CornerType::None => return None,
            CornerType::Closed => CLOSED_BOTTOM_RIGHT_CORNER_PATH.to_string(),
            CornerType::Open => OPEN_BOTTOM_RIGHT_CORNER_PATH.to_string(),
            CornerType::Vertical => RIGHT_BORDER_PATH.to_string(),
            CornerType::Horizontal => BOTTOM_BORDER_PATH.to_string(),
        },
        CornerPosition::BottomLeft => match corner_type {
            CornerType::None => return None,
            CornerType::Closed => CLOSED_BOTTOM_LEFT_CORNER_PATH.to_string(),
            CornerType::Open => OPEN_BOTTOM_LEFT_CORNER_PATH.to_string(),
            CornerType::Vertical => LEFT_BORDER_PATH.to_string(),
            CornerType::Horizontal => BOTTOM_BORDER_PATH.to_string(),
        },
    };

    let location = match position {
        CornerPosition::TopLeft => dcl2d_ecs_v1::Vec2 {
            x: PARCEL_SIZE_X as i32 / -2,
            y: PARCEL_SIZE_Y as i32 / 2,
        },
        CornerPosition::TopRight => dcl2d_ecs_v1::Vec2 {
            x: PARCEL_SIZE_X as i32 / 2,
            y: PARCEL_SIZE_Y as i32 / 2,
        },
        CornerPosition::BottomRight => dcl2d_ecs_v1::Vec2 {
            x: PARCEL_SIZE_X as i32 / 2,
            y: PARCEL_SIZE_Y as i32 / -2,
        },
        CornerPosition::BottomLeft => dcl2d_ecs_v1::Vec2 {
            x: PARCEL_SIZE_X as i32 / -2,
            y: PARCEL_SIZE_Y as i32 / -2,
        },
    };

    let transform = dcl2d_ecs_v1::components::Transform {
        location,
        rotation: dcl2d_ecs_v1::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
    };

    let anchor = match position {
        CornerPosition::TopLeft => dcl2d_ecs_v1::Anchor::TopLeft,
        CornerPosition::TopRight => dcl2d_ecs_v1::Anchor::TopRight,
        CornerPosition::BottomLeft => dcl2d_ecs_v1::Anchor::BottomLeft,
        CornerPosition::BottomRight => dcl2d_ecs_v1::Anchor::BottomRight,
    };

    let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
        sprite,
        layer: -1,
        anchor,
        ..default()
    };

    Some(dcl2d_ecs_v1::Entity {
        name: "Corner".to_string(),
        components: vec![Box::new(renderer), Box::new(transform)],
        ..default()
    })
}

fn make_road_background_entities() -> Vec<dcl2d_ecs_v1::Entity> {
    let mut entities = Vec::new();
    let total_tiles_x = PARCEL_SIZE_X as i32 / (TILE_SIZE.0 * 4);
    let total_tiles_y = PARCEL_SIZE_Y as i32 / (TILE_SIZE.1 * 4);

    for x in 0..total_tiles_x {
        for y in 0..total_tiles_y {
            let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                sprite: ROAD_BACKGROUND_PATH.to_string(),
                layer: -3,
                anchor: dcl2d_ecs_v1::Anchor::BottomLeft,
                ..default()
            };

            let location_x = TILE_SIZE.0 * x * 4 - PARCEL_SIZE_X as i32 / 2;
            let location_y = TILE_SIZE.1 * y * 4 - PARCEL_SIZE_Y as i32 / 2;
            let transform = dcl2d_ecs_v1::components::Transform {
                location: dcl2d_ecs_v1::Vec2 {
                    x: location_x,
                    y: location_y,
                },
                rotation: dcl2d_ecs_v1::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
            };

            entities.push(dcl2d_ecs_v1::Entity {
                name: "Background".to_string(),
                components: vec![Box::new(renderer), Box::new(transform)],
                ..default()
            });
        }
    }

    entities
}

fn make_default_background_entities() -> Vec<dcl2d_ecs_v1::Entity> {
    let mut entities = Vec::new();
    let total_tiles_x = PARCEL_SIZE_X as i32 / TILE_SIZE.0;
    let total_tiles_y = PARCEL_SIZE_Y as i32 / TILE_SIZE.1;
    let mut rng = rand::thread_rng();
    let mut bg_files = Vec::default();
    let bg_path =
        std::fs::read_dir("./assets/default_scene/assets/".to_string() + DEFAULT_BACKGROUND_PATH)
            .unwrap();

    for path in bg_path.flatten() {
        if let Some(str) = path.file_name().to_str() {
            bg_files.push(DEFAULT_BACKGROUND_PATH.to_string() + "/" + str);
        }
    }

    for x in 0..total_tiles_x {
        for y in 0..total_tiles_y {
            let random_bg_index: usize = rng.gen_range(0..bg_files.len() - 1);
            let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                sprite: bg_files[random_bg_index].clone(),
                layer: -3,
                anchor: dcl2d_ecs_v1::Anchor::BottomLeft,
                ..default()
            };

            let location_x = TILE_SIZE.0 * x - PARCEL_SIZE_X as i32 / 2;
            let location_y = TILE_SIZE.1 * y - PARCEL_SIZE_Y as i32 / 2;
            let transform = dcl2d_ecs_v1::components::Transform {
                location: dcl2d_ecs_v1::Vec2 {
                    x: location_x,
                    y: location_y,
                },
                rotation: dcl2d_ecs_v1::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
            };

            entities.push(dcl2d_ecs_v1::Entity {
                name: "Background".to_string(),
                components: vec![Box::new(renderer), Box::new(transform)],
                ..default()
            });
        }
    }

    entities
}

fn make_default_random_entities() -> Vec<dcl2d_ecs_v1::Entity> {
    let mut entities = Vec::new();
    let mut rng = rand::thread_rng();

    let no_collision_paths = std::fs::read_dir(
        "./assets/default_scene/assets/".to_string() + DEFAULT_RANDOM_NO_COLLISION,
    )
    .unwrap();
    let mut no_collision_obstacles = Vec::default();

    let small_collision_paths = std::fs::read_dir(
        "./assets/default_scene/assets/".to_string() + DEFAULT_RANDOM_OBSTACLE_SIZE_1,
    )
    .unwrap();
    let mut small_collision_obstacles = Vec::default();

    let big_collision_paths = std::fs::read_dir(
        "./assets/default_scene/assets/".to_string() + DEFAULT_RANDOM_OBSTACLE_SIZE_2,
    )
    .unwrap();
    let mut big_collision_obstacles = Vec::default();

    for path in no_collision_paths.flatten() {
        if let Some(str) = path.file_name().to_str() {
            no_collision_obstacles.push(DEFAULT_RANDOM_NO_COLLISION.to_string() + "/" + str);
        }
    }

    for path in small_collision_paths.flatten() {
        if let Some(str) = path.file_name().to_str() {
            small_collision_obstacles.push(DEFAULT_RANDOM_OBSTACLE_SIZE_1.to_string() + "/" + str);
        }
    }

    for path in big_collision_paths.flatten() {
        if let Some(str) = path.file_name().to_str() {
            big_collision_obstacles.push(DEFAULT_RANDOM_OBSTACLE_SIZE_2.to_string() + "/" + str);
        }
    }

    for random_stuff in no_collision_obstacles {
        if rng.gen_bool(RANDOM_OBSTACLE_SPAWN_CHANCE) {
            if let Ok(mut base_path) = PathBuf::from_str("./assets/default_scene/assets/") {
                base_path.push(&random_stuff);
                let png_files = std::fs::read_dir(base_path).unwrap();

                let location_x = rng.gen_range((PARCEL_SIZE_X * 0.8/ -2.0 )as i32 ..(PARCEL_SIZE_X * 0.8/ 2.0 ) as i32);
                let location_y = rng.gen_range((PARCEL_SIZE_Y * 0.8/ -2.0 )as i32..(PARCEL_SIZE_Y * 0.8/ 2.0 ) as i32);
                let transform = dcl2d_ecs_v1::components::Transform {
                    location: dcl2d_ecs_v1::Vec2 {
                        x: location_x,
                        y: location_y,
                    },
                    rotation: dcl2d_ecs_v1::Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                };

                for png_file in png_files.flatten() {
                    if let Some(png_file_str) = png_file.file_name().to_str() {
                      
                        let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                            sprite: random_stuff.clone() + "/" + png_file_str,
                            layer: -1,
                            ..default()
                        };

                        entities.push(dcl2d_ecs_v1::Entity {
                            name: png_file_str.to_string(),
                            components: vec![Box::new(renderer), Box::new(transform.clone())],
                            ..default()
                        });
                    }
                }
            }
        }
    }

    for random_stuff in small_collision_obstacles {
        if rng.gen_bool(RANDOM_OBSTACLE_SPAWN_CHANCE) {
            if let Ok(mut base_path) = PathBuf::from_str("./assets/default_scene/assets/") {
                base_path.push(&random_stuff);
                let png_files = std::fs::read_dir(base_path).unwrap();

                let location_x = rng.gen_range((PARCEL_SIZE_X * 0.8/ -2.0 )as i32 ..(PARCEL_SIZE_X * 0.8/ 2.0 ) as i32);
                let location_y = rng.gen_range((PARCEL_SIZE_Y * 0.8/ -2.0 )as i32..(PARCEL_SIZE_Y * 0.8/ 2.0 ) as i32);                 let transform = dcl2d_ecs_v1::components::Transform {
                    location: dcl2d_ecs_v1::Vec2 {
                        x: location_x,
                        y: location_y,
                    },
                    rotation: dcl2d_ecs_v1::Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                };

                let box_collision = dcl2d_ecs_v1::components::BoxCollider {
                    collision_type: dcl2d_ecs_v1::collision_type::CollisionType::Solid,
                    center: dcl2d_ecs_v1::Vec2 { x: 0, y: 0 },
                    size: dcl2d_ecs_v1::Size {
                        width: 25,
                        height: 25,
                    },
                };

                for png_file in png_files.flatten() {
                    if let Some(png_file_str) = png_file.file_name().to_str() {
                        let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                            sprite: random_stuff.clone() + "/" + png_file_str,
                            ..default()
                        };

                        entities.push(dcl2d_ecs_v1::Entity {
                            name: png_file_str.to_string(),
                            components: vec![
                                Box::new(renderer),
                                Box::new(transform.clone()),
                                Box::new(box_collision.clone()),
                            ],
                            ..default()
                        });
                    }
                }
            }
        }
    }

    for random_stuff in big_collision_obstacles {
        if rng.gen_bool(RANDOM_OBSTACLE_SPAWN_CHANCE) {
            if let Ok(mut base_path) = PathBuf::from_str("./assets/default_scene/assets/") {
                base_path.push(&random_stuff);
                let png_files = std::fs::read_dir(base_path).unwrap();
                let location_x = rng.gen_range((PARCEL_SIZE_X * 0.8/ -2.0 )as i32 ..(PARCEL_SIZE_X * 0.8/ 2.0 ) as i32);
                let location_y = rng.gen_range((PARCEL_SIZE_Y * 0.8/ -2.0 )as i32..(PARCEL_SIZE_Y * 0.8/ 2.0 ) as i32);
                  let transform = dcl2d_ecs_v1::components::Transform {
                    location: dcl2d_ecs_v1::Vec2 {
                        x: location_x,
                        y: location_y,
                    },
                    rotation: dcl2d_ecs_v1::Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    scale: dcl2d_ecs_v1::Vec2 { x: 1.0, y: 1.0 },
                };

                let box_collision = dcl2d_ecs_v1::components::BoxCollider {
                    collision_type: dcl2d_ecs_v1::collision_type::CollisionType::Solid,
                    center: dcl2d_ecs_v1::Vec2 { x: 0, y: 0 },
                    size: dcl2d_ecs_v1::Size {
                        width: 75,
                        height: 75,
                    },
                };

                for png_file in png_files.flatten() {
                    if let Some(png_file_str) = png_file.file_name().to_str() {
                        let renderer = dcl2d_ecs_v1::components::SpriteRenderer {
                            sprite: random_stuff.clone() + "/" + png_file_str,
                            ..default()
                        };

                        entities.push(dcl2d_ecs_v1::Entity {
                            name: png_file_str.to_string(),
                            components: vec![
                                Box::new(renderer),
                                Box::new(transform.clone()),
                                Box::new(box_collision.clone()),
                            ],
                            ..default()
                        });
                    }
                }
            }
        }
    }
    entities
}

pub fn read_roads_data() -> Result<RoadsData> {
    let file = match File::open(&ROADS_DATA_MP_FILE) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };
    let reader = BufReader::new(file);
    let mut de = Deserializer::new(reader);

    let serializable_roads_data: SerializableRoadsData = match Deserialize::deserialize(&mut de) {
        Ok(result) => result,
        Err(e) => return Err(Box::new(e)),
    };

    let mut roads_data = RoadsData::default();

    for parcel in serializable_roads_data.parcels {
        roads_data.parcel_map.insert((parcel.0, parcel.1), ());
    }

    Ok(roads_data)
}
/*
pub fn update_roads_data(new_roads_data: &RoadsData) -> Result<()> {
    let mut serializable_roads_data = SerializableRoadsData::default();

    for key in new_roads_data.parcel_map.keys() {
        serializable_roads_data.parcels.push(Parcel(key.0, key.1));
    }

    let mut buf: Vec<u8> = Vec::new();
    match serializable_roads_data.serialize(&mut Serializer::new(&mut buf)) {
        Ok(_) => {}
        Err(e) => return Err(Box::new(e)),
    }

    let mut file = match File::create(&ROADS_DATA_MP_FILE) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    match file.write_all(&buf) {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn remove_road_at_parcel(parcel: &Parcel, roads_data: &mut RoadsData) -> Result<()> {
    roads_data.parcel_map.remove(&(parcel.0, parcel.1));
    update_roads_data(roads_data)
}

pub fn add_road_at_parcel(parcel: &Parcel, roads_data: &mut RoadsData) -> Result<()> {
    roads_data.parcel_map.insert((parcel.0, parcel.1), ());
    update_roads_data(roads_data)
} */

pub fn is_road(parcel: &Parcel, roads_data: &RoadsData) -> bool {
    roads_data.parcel_map.get(&(parcel.0, parcel.1)).is_some()
}
