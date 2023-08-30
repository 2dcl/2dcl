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
const DEFAULT_JUMP_BUTTON: &str = "ui/discover_default.png";
const HOVERED_JUMP_BUTTON: &str = "ui/discover_hovered.png";
const PRESSED_JUMP_BUTTON: &str = "ui/discover_pressed.png";
const BG_COLOR: Color = Color::rgb(0.098, 0.075, 0.102);
const TEXT_COLOR: Color = Color::WHITE;
const HEADERS_COLOR: Color = Color::RED;

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
            .add_systems(Update, button_system.run_if(in_state(AppState::InGame)));
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
                width: Val::Px(138.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            image: UiImage::new(discover_button.0.default.clone()),
            ..Default::default()
        },
        discover_button,
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
            NodeBundle {
                background_color: BackgroundColor(BG_COLOR),
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::all(Val::Percent(5.)),
                    width: Val::Percent(90.),
                    height: Val::Percent(90.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            DiscoverUI,
        ))
        .id();

    let headers = spawn_entry(
        commands,
        &font,
        HEADERS_COLOR,
        18.0,
        vec![
            "Scene".to_string(),
            "Parcel".to_string(),
            "Last Update".to_string(),
            "Jump".to_string(),
        ],
    );
    commands.entity(headers).set_parent(discover_ui);

    for scene in scenes {
        let entry = spawn_scene_entry(commands, &font, &scene, button_settings.clone());
        commands.entity(entry).set_parent(discover_ui);
    }
}

fn spawn_entry(
    commands: &mut Commands,
    font: &Handle<Font>,
    color: Color,
    font_size: f32,
    columns: Vec<String>,
) -> Entity {
    let entry = commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    for column in columns {
        commands
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
                    margin: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .set_parent(entry);
    }

    entry
}

fn spawn_scene_entry(
    commands: &mut Commands,
    font: &Handle<Font>,
    scene: &SceneDiscoveryData,
    button_settings: ButtonSettings,
) -> Entity {
    let entry = spawn_entry(
        commands,
        font,
        TEXT_COLOR,
        16.0,
        vec![
            scene.title.clone(),
            scene.get_parcel_str(),
            scene.pub_date.clone(),
        ],
    );

    if let Ok(parcel) = scene.get_parcel() {
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(138.),
                        height: Val::Px(50.),
                        ..Default::default()
                    },
                    image: UiImage::new(button_settings.default.clone()),
                    ..Default::default()
                },
                JumpButton {
                    settings: button_settings,
                    parcel,
                },
            ))
            .set_parent(entry);
    };

    entry
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
                image.texture = button.settings.pressed.clone();
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
