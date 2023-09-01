use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use dcl_common::{Parcel, Result};

use crate::components;
use crate::content_discovery::{find_2d_scenes, SceneDiscoveryData};
use crate::renderer::scene_loader::get_parcel_spawn_point;
use crate::states::AppState;

use super::scene_maker::RoadsData;
use super::scenes_io::SceneFilesMap;

const DEFAULT_DISCOVER_BUTTON: &str = "ui/discover_default.png";
const HOVERED_DISCOVER_BUTTON: &str = "ui/discover_hovered.png";
const PRESSED_DISCOVER_BUTTON: &str = "ui/discover_pressed.png";
const DEFAULT_JUMP_BUTTON: &str = "ui/go_default.png";
const HOVERED_JUMP_BUTTON: &str = "ui/go_hovered.png";
const PRESSED_JUMP_BUTTON: &str = "ui/go_hovered.png";
const BG_COLOR: Color = Color::rgba(1., 1., 1., 0.95);
const TEXT_COLOR: Color = Color::WHITE;
const HEADER_COLOR: Color = Color::RED;
const HEADER_FONT_SIZE: f32 = 18.0;
const SCENES_FONT_SIZE: f32 = 16.0;
const JUMP_BUTTON_HEIGHT: f32 = 40.0;

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component)]
struct DiscoverUI;
#[derive(Component)]
struct DiscoverButton(pub ButtonSettings);

#[derive(Component, Clone)]
struct JumpButton {
    settings: ButtonSettings,
    parcel: Parcel,
}

#[derive(Clone)]
struct ButtonSettings {
    default: Handle<Image>,
    pressed: Handle<Image>,
    hovered: Handle<Image>,
}
pub struct DiscoveryUiPlugin;

impl Plugin for DiscoveryUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (button_system, mouse_scroll).run_if(in_state(AppState::InGame)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let discover_button = DiscoverButton(ButtonSettings {
        default: asset_server.load(DEFAULT_DISCOVER_BUTTON),
        pressed: asset_server.load(PRESSED_DISCOVER_BUTTON),
        hovered: asset_server.load(HOVERED_DISCOVER_BUTTON),
    });

    commands.spawn((
        ButtonBundle {
            style: Style {
                height: Val::Percent(5.),
                aspect_ratio: Some(2.76),
                margin: UiRect {
                    left: Val::Px(8.),
                    top: Val::Px(8.),
                    ..Default::default()
                },
                ..Default::default()
            },
            image: UiImage::new(discover_button.0.default.clone()),
            ..Default::default()
        },
        discover_button,
        Name::new("Discover button"),
    ));
}

fn show_discover_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scenes: Vec<SceneDiscoveryData>,
) {
    let button_settings = ButtonSettings {
        default: asset_server.load(DEFAULT_JUMP_BUTTON),
        pressed: asset_server.load(PRESSED_JUMP_BUTTON),
        hovered: asset_server.load(HOVERED_JUMP_BUTTON),
    };

    let font = asset_server.load("fonts/Arcadepix Plus.ttf");
    let discover_ui = commands
        .spawn((
            ImageBundle {
                image: UiImage::new(asset_server.load("ui/back.png")),
                background_color: BackgroundColor(BG_COLOR),
                style: Style {
                    border: UiRect::all(Val::Px(10.)),
                    position_type: PositionType::Absolute,
                    width: Val::Percent(70.),
                    height: Val::Percent(70.),
                    margin: UiRect::all(Val::Percent(15.)),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    ..Default::default()
                },
                ..Default::default()
            },
            DiscoverUI,
            Name::new("Discover UI"),
        ))
        .id();

    let header = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: vec![RepeatedGridTrack::fr(4, 1.)],
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new("Header"),
        ))
        .set_parent(discover_ui)
        .id();

    spawn_entry(
        commands,
        header,
        &font,
        HEADER_COLOR,
        HEADER_FONT_SIZE,
        vec![
            "SCENE".to_string(),
            "PARCEL".to_string(),
            "LAST UPDATE".to_string(),
            "JUMP".to_string(),
        ],
        false,
    );
    let scenes_ui = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: vec![RepeatedGridTrack::fr(4, 1.)],
                    //  grid_template_rows: vec![RepeatedGridTrack::px(1, SCENES_FONT_SIZE)],
                    overflow: Overflow::clip_y(),
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new("Scenes"),
        ))
        .set_parent(discover_ui)
        .id();

    for scene in scenes {
        spawn_scene_entry(commands, scenes_ui, &font, &scene, button_settings.clone());
    }
}

fn spawn_entry(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    color: Color,
    font_size: f32,
    columns: Vec<String>,
    scrollable: bool,
) {
    for column in columns {
        let entry = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    column.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                ),
                style: Style {
                    height: Val::Px(SCENES_FONT_SIZE),
                    margin: UiRect::all(Val::Px(5.)),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .set_parent(parent)
            .id();

        if scrollable {
            commands.entity(entry).insert(ScrollingList::default());
        }
    }
}

fn spawn_scene_entry(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    scene: &SceneDiscoveryData,
    button_settings: ButtonSettings,
) {
    spawn_entry(
        commands,
        parent,
        font,
        TEXT_COLOR,
        SCENES_FONT_SIZE,
        vec![
            scene.title.clone(),
            scene.get_parcel_str(),
            scene.pub_date.clone(),
        ],
        true,
    );

    if let Ok(parcel) = scene.get_parcel() {
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        height: Val::Px(JUMP_BUTTON_HEIGHT),
                        aspect_ratio: Some(1.66),
                        margin: UiRect::all(Val::Px(5.)),
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    image: UiImage::new(button_settings.default.clone()),
                    ..Default::default()
                },
                JumpButton {
                    settings: button_settings,
                    parcel,
                },
                ScrollingList::default(),
            ))
            .set_parent(parent);
    };
}

fn button_system(
    mut discover_query: Query<
        (&Interaction, &mut UiImage, &DiscoverButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut jump_query: Query<
        (&Interaction, &mut UiImage, &JumpButton),
        (Changed<Interaction>, With<Button>, Without<DiscoverButton>),
    >,
    mut player_query: Query<(&mut components::Player, &mut Transform)>,
    mut roads_data: ResMut<RoadsData>,
    scene_files_map: Res<SceneFilesMap>,
    discover_ui_query: Query<Entity, With<DiscoverUI>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut image, button) in &mut discover_query {
        match *interaction {
            Interaction::Pressed => {
                image.texture = button.0.pressed.clone();
                if let Ok(discover_ui) = discover_ui_query.get_single() {
                    commands.entity(discover_ui).despawn_recursive();
                } else if let Ok(scenes) = get_2d_scenes() {
                    show_discover_ui(&mut commands, &asset_server, scenes);
                }
            }
            Interaction::Hovered => {
                image.texture = button.0.hovered.clone();
            }
            Interaction::None => {
                image.texture = button.0.default.clone();
            }
        }
    }

    for (interaction, mut image, button) in &mut jump_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(discover_ui) = discover_ui_query.get_single() {
                    commands.entity(discover_ui).despawn_recursive();
                }
                image.texture = button.settings.pressed.clone();
                let (mut player, mut transform) = player_query.single_mut();
                player.current_level = 0;
                transform.translation =
                    get_parcel_spawn_point(&button.parcel, 0, &mut roads_data, &scene_files_map);
                let (mut player, mut transform) = player_query.single_mut();
                player.current_level = 0;
                transform.translation =
                    get_parcel_spawn_point(&button.parcel, 0, &mut roads_data, &scene_files_map);
            }
            Interaction::Hovered => {
                image.texture = button.settings.hovered.clone();
            }
            Interaction::None => {
                image.texture = button.settings.default.clone();
            }
        }
    }
}

#[tokio::main]
async fn get_2d_scenes() -> Result<Vec<SceneDiscoveryData>> {
    find_2d_scenes().await
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent) in &mut query_list {
            let container_height = query_node.get(parent.get()).unwrap().size().y;
            let entry_height = JUMP_BUTTON_HEIGHT.max(SCENES_FONT_SIZE);
            let max_scroll = (container_height - entry_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
