use crate::{renderer::update_avatar, resources::Config, states::AppState};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use ethereum_adapter::{EthAddress, EthereumAdapter};
use futures_lite::future;
use std::{thread, time};
pub struct MetamaskLoginPlugin;

#[derive(Component)]
struct WebLogin(Task<Option<EthAddress>>);

#[derive(Component)]
struct DisplayText;

impl Plugin for MetamaskLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MetamaskLogin), setup)
            .add_systems(
                Update,
                (button_system, handle_tasks).run_if(in_state(AppState::MetamaskLogin)),
            )
            .add_systems(OnExit(AppState::MetamaskLogin), exit);
    }
}

const CLEAR_COLOR: Color = Color::rgb(0.12, 0.1, 0.25);

const NORMAL_BUTTON: Color = Color::rgb(0.43, 0.04, 0.12);
const HOVERED_BUTTON: Color = Color::CRIMSON;
const PRESSED_BUTTON: Color = Color::rgb(1.72, 0.16, 0.48);

const NORMAL_TEXT: Color = Color::rgb(0.85, 0.85, 0.85);
const HOVERED_TEXT: Color = Color::WHITE;
const PRESSED_TEXT: Color = Color::WHITE;

const NORMAL_BORDER: Color = Color::BLACK;
const HOVERED_BORDER: Color = Color::WHITE;
const PRESSED_BORDER: Color = Color::WHITE;

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Style,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut display_text_query: Query<&mut Style, (With<DisplayText>, Without<Interaction>)>,
) {
    for (interaction, mut color, mut border_color, mut style, children) in &mut interaction_query {
        let mut text: Mut<'_, Text> = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                style.display = Display::None;
                *color = PRESSED_BUTTON.into();
                border_color.0 = PRESSED_BORDER;

                if let Ok(mut display_text) = display_text_query.get_single_mut() {
                    display_text.display = Display::Flex;
                }
                // Spawn new task on the AsyncComputeTaskPool
                let thread_pool = AsyncComputeTaskPool::get();
                let task = thread_pool.spawn(async move { login().unwrap() });
                // Spawn new entity and add our new task as a component
                commands.spawn(WebLogin(task));
                text.sections[0].style.color = PRESSED_TEXT;

                //SHOW TEXT
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = HOVERED_BORDER;
                text.sections[0].style.color = HOVERED_TEXT;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = NORMAL_BORDER;
                text.sections[0].style.color = NORMAL_TEXT;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
        },
        ..Default::default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(186.0),
                        height: Val::Px(183.0),
                        margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(asset_server.load("ui/2dcl_logo.png")),
            ));
            parent
                .spawn(TextBundle {
                    style: Style {
                        display: Display::None,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Continue the login process in your browser.",
                        TextStyle {
                            font: asset_server.load("fonts/kongtext.ttf"),
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                    ..Default::default()
                })
                .insert(DisplayText);
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: NORMAL_BORDER.into(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Login",
                        TextStyle {
                            font: asset_server.load("fonts/kongtext.ttf"),
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn exit(
    mut commands: Commands,
    ui_elements: Query<Entity, &Node>,
    cameras: Query<Entity, &Camera>,
    config: Res<Config>,
) {
    for ui in ui_elements.into_iter() {
        commands.entity(ui).despawn_recursive();
    }
    for camera in cameras.into_iter() {
        commands.entity(camera).despawn_recursive();
    }
    update_avatar(&config.avatar.eth_address);
}

fn handle_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut WebLogin)>,
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
    mut display_text_query: Query<&mut Text, With<DisplayText>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(address) = future::block_on(future::poll_once(&mut task.0)) {
            // Add our new PbrBundle of components to our tagged entity
            if let Some(address) = address {
                if let Ok(mut display_text) = display_text_query.get_single_mut() {
                    display_text.sections[0].value = "Making avatar".to_string();
                }
                config.avatar.eth_address = address;
                next_state.set(AppState::InGame);
            }
            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<WebLogin>();
        }
    }
}

#[tokio::main]
async fn login() -> dcl_common::Result<Option<EthAddress>> {
    let mut adapter = EthereumAdapter::default();
    let mut command = std::env::current_exe().unwrap();
    command.pop();
    adapter.start(&mut command).unwrap();

    adapter.login();
    println!("Waiting...");
    while !adapter.is_logged_in().await {
        thread::sleep(time::Duration::from_millis(1000));
        println!("Awaiting for login...");
    }
    Ok(adapter.address())
}
