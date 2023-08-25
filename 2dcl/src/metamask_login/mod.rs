use crate::{renderer::update_avatar, resources::Config, states::AppState};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use ethereum_adapter::{EthAddress, EthereumAdapter};
use futures_lite::future;
use std::{thread, time};

const CLEAR_COLOR: Color = Color::rgb(0.098, 0.075, 0.102);
const TEXT_COLOR: Color = Color::rgb(1.0, 0.176, 0.333);
const DCL_2D_LOGO: &str = "ui/login_screen/2dcl_LOGO.png";
const DEFAULT_BUTTON: &str = "ui/login_screen/LOGIN.png";
const HOVERED_BUTTON: &str = "ui/login_screen/LOGIN_.png";
const DCL_LOGO: &str = "ui/login_screen/LOGO_DCL.png";
const LOADING_ICON: &str = "ui/login_screen/loading.png";
const LOADING_ANIMATION_SPRITE_COUNT: usize = 8;

pub struct MetamaskLoginPlugin;

#[derive(Component)]
struct WebLogin(Task<Option<EthAddress>>);

#[derive(Component)]
struct AvatarMaker(Task<()>);

#[derive(Component)]
struct LoginButton {
    default: Handle<Image>,
    hovered: Handle<Image>,
}

#[derive(Component)]
struct DCL2dLogo;

#[derive(Component, Deref, DerefMut, Clone)]
pub struct LoadingIcon(Timer);

impl LoadingIcon {
    pub fn new(duration: f32) -> Self {
        LoadingIcon(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

#[derive(Component)]
enum DisplayText {
    WatingForInput,
    WebLogin,
    MakingAvatar { animation_timer: Timer },
    Loading2dcl { animation_timer: Timer },
}

impl Plugin for MetamaskLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MetamaskLogin), setup)
            .add_systems(
                Update,
                (
                    button_system,
                    handle_tasks,
                    display_text,
                    animate_loading_screen,
                )
                    .run_if(in_state(AppState::MetamaskLogin)),
            )
            .add_systems(OnExit(AppState::MetamaskLogin), exit);
    }
}

fn display_text(mut display_text_query: Query<(&mut DisplayText, &mut Text)>, time: Res<Time>) {
    if let Ok((mut display_text, mut text)) = display_text_query.get_single_mut() {
        text.sections[0].value = match display_text.as_mut() {
            DisplayText::WatingForInput => "2023 DECENTRALAND AND HIDDEN PEOPLE CLUB.".to_string(),
            DisplayText::WebLogin => "Continue the login process in your browser.".to_string(),
            DisplayText::MakingAvatar { animation_timer } => {
                animation_timer.tick(time.delta());
                format!(
                    "Making avatar{}",
                    get_dots(&text.sections[0].value, animation_timer)
                )
            }
            DisplayText::Loading2dcl { animation_timer } => {
                animation_timer.tick(time.delta());
                format!(
                    "Loading 2dcl{}",
                    get_dots(&text.sections[0].value, animation_timer)
                )
            }
        };
    }

    fn get_dots(previuos_string: &str, timer: &Timer) -> String {
        let mut total_dots = previuos_string.matches('.').count();
        if timer.just_finished() {
            total_dots += 1;
            if total_dots > 3 {
                total_dots = 1;
            }
        }

        let mut final_string = String::default();
        for _ in 0..total_dots {
            final_string += ".";
        }

        final_string
    }
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut Style, &mut UiImage, &LoginButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut dcl_2d_icon: Query<Entity, (With<DCL2dLogo>, Without<Button>, Without<LoadingIcon>)>,
    mut display_text_query: Query<&mut DisplayText>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut text_query: Query<&mut Style, (With<Text>, Without<Button>, Without<DCL2dLogo>)>,
) {
    for (interaction, mut style, mut image, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                style.display = Display::None;
                if let Ok(mut display_text) = display_text_query.get_single_mut() {
                    *display_text = DisplayText::WebLogin;
                }

                if let Ok(mut text_style) = text_query.get_single_mut() {
                    text_style.bottom = Val::Percent(75.);
                }

                if let Ok(entity) = dcl_2d_icon.get_single_mut() {
                    commands.entity(entity).remove::<UiImage>();
                    commands.entity(entity).remove::<DCL2dLogo>();
                    let texture_handle = asset_server.load(LOADING_ICON);
                    let texture_atlas = TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(114., 89.),
                        4,
                        2,
                        None,
                        None,
                    );
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);
                    commands
                        .entity(entity)
                        .insert(AtlasImageBundle {
                            texture_atlas: texture_atlas_handle,
                            style: Style {
                                width: Val::Px(114.0),
                                height: Val::Px(89.0),
                                top: Val::Percent(25.),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(LoadingIcon::new(0.1));
                }

                let thread_pool = AsyncComputeTaskPool::get();
                let task = thread_pool.spawn(async move { login().unwrap() });
                commands.spawn(WebLogin(task));
            }
            Interaction::Hovered => {
                image.texture = button.hovered.clone();
            }
            Interaction::None => {
                image.texture = button.default.clone();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let login_button = LoginButton {
        default: asset_server.load(DEFAULT_BUTTON),
        hovered: asset_server.load(HOVERED_BUTTON),
    };

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
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(246.0),
                        height: Val::Px(120.0),
                        margin: UiRect::top(Val::VMin(25.)),
                        ..default()
                    },
                    // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(asset_server.load(DCL_2D_LOGO)),
                DCL2dLogo,
            ));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        align_self: AlignSelf::Stretch,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(107.33),
                                height: Val::Px(43.33),
                                bottom: Val::Percent(50.),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: UiImage::new(login_button.default.clone()),
                            ..default()
                        },
                        login_button,
                    ));
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: asset_server.load("fonts/Arcadepix Plus.ttf"),
                                    font_size: 16.0,
                                    color: TEXT_COLOR,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(DisplayText::WatingForInput);

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(130.0),
                                height: Val::Px(128.66),
                                justify_self: JustifySelf::End,
                                align_self: AlignSelf::End,
                                right: Val::Px(10.),
                                bottom: Val::Px(10.),
                                ..default()
                            },
                            // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load(DCL_LOGO)),
                    ));
                });
        });
}

fn exit(
    mut commands: Commands,
    ui_elements: Query<Entity, &Node>,
    cameras: Query<Entity, &Camera>,
) {
    for ui in ui_elements.into_iter() {
        commands.entity(ui).despawn_recursive();
    }
    for camera in cameras.into_iter() {
        commands.entity(camera).despawn_recursive();
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut web_login_tasks: Query<(Entity, &mut WebLogin)>,
    mut avatar_maker_tasks: Query<(Entity, &mut AvatarMaker)>,
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
    mut display_text_query: Query<&mut DisplayText>,
) {
    for (entity, mut task) in &mut web_login_tasks {
        if let Some(address) = future::block_on(future::poll_once(&mut task.0)) {
            // Add our new PbrBundle of components to our tagged entity
            if let Some(address) = address {
                if let Ok(mut display_text) = display_text_query.get_single_mut() {
                    *display_text = DisplayText::MakingAvatar {
                        animation_timer: Timer::from_seconds(0.75, TimerMode::Repeating),
                    };
                }
                config.avatar.eth_address = address.clone();
                let thread_pool = AsyncComputeTaskPool::get();
                let task = thread_pool.spawn(async move { update_avatar(&address) });
                commands.spawn(AvatarMaker(task));
            }
            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<WebLogin>();
        }
    }

    for (entity, mut task) in &mut avatar_maker_tasks {
        if future::block_on(future::poll_once(&mut task.0)).is_some() {
            if let Ok(mut display_text) = display_text_query.get_single_mut() {
                *display_text = DisplayText::Loading2dcl {
                    animation_timer: Timer::from_seconds(0.75, TimerMode::Repeating),
                };
            }
            next_state.set(AppState::InGame);
            commands.entity(entity).remove::<AvatarMaker>();
        }
    }
}

fn animate_loading_screen(
    time: Res<Time>,
    mut query: Query<(&mut LoadingIcon, &mut UiTextureAtlasImage)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= LOADING_ANIMATION_SPRITE_COUNT - 1 {
                0
            } else {
                sprite.index + 1
            };
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

    adapter.stop().await?;

    Ok(adapter.address())
}

#[cfg(test)]
mod test {
    use super::{handle_tasks, WebLogin};
    use crate::{metamask_login::AvatarMaker, resources::Config, states::AppState};
    use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
    use ethereum_adapter::EthAddress;

    #[test]
    fn eth_address_updates_when_web_task_finishes() {
        let mut app = App::new();
        app.add_state::<AppState>();
        app.add_plugins(bevy::prelude::TaskPoolPlugin::default());
        app.insert_resource(Config::default());
        app.add_systems(Update, handle_tasks);

        let thread_pool = AsyncComputeTaskPool::get();
        let new_address = EthAddress {
            address: "new_address".to_string(),
        };
        let new_address_clone = new_address.clone();

        let task = thread_pool.spawn(async move { Some(new_address_clone) });
        app.world.spawn(WebLogin(task));
        app.update();

        let current_address = &app.world.resource::<Config>().avatar.eth_address;
        assert_eq!(new_address, *current_address);
    }

    #[test]
    fn changes_states_after_avatar_making_finishes() {
        let mut app = App::new();
        app.add_state::<AppState>();
        app.add_plugins(bevy::prelude::TaskPoolPlugin::default());
        app.insert_resource(Config::default());
        app.add_systems(Update, handle_tasks);

        let thread_pool = AsyncComputeTaskPool::get();

        let task = thread_pool.spawn(async move { () });
        app.world.spawn(AvatarMaker(task));
        app.update();
        app.update();

        let current_state = app.world.get_resource::<State<AppState>>().unwrap().get();
        assert_eq!(AppState::InGame, *current_state);
    }
}
