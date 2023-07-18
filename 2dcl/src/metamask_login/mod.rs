use crate::{renderer::update_avatar, resources::Config, states::AppState};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use ethereum_adapter::{EthAddress, EthereumAdapter};
use futures_lite::future;
use std::{thread, time};
pub struct MetamaskLoginPlugin;

#[derive(Component)]
struct WebLogin(Task<Option<EthAddress>>);

impl Plugin for MetamaskLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MetamaskLogin), login_ui)
            .add_systems(
                Update,
                (button_system, handle_tasks).run_if(in_state(AppState::MetamaskLogin)),
            )
            .add_systems(OnExit(AppState::MetamaskLogin), despawn_login_ui);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                // Spawn new task on the AsyncComputeTaskPool
                let thread_pool = AsyncComputeTaskPool::get();
                let task = thread_pool.spawn(async move { login().unwrap() });

                // Spawn new entity and add our new task as a component
                commands.spawn(WebLogin(task));
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn login_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}


fn despawn_login_ui(mut commands: Commands, ui_elements: Query<Entity, &Node>, cameras: Query<Entity, &Camera>) {
 for ui in ui_elements.into_iter()
 {
    commands.entity(ui).despawn_recursive();
 }
 for camera in cameras.into_iter()
 {
    commands.entity(camera).despawn_recursive();
 }
}

fn handle_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut WebLogin)>,
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(address) = future::block_on(future::poll_once(&mut task.0)) {
            // Add our new PbrBundle of components to our tagged entity
            println!("This is my address: {:?}", address);
            if let Some(address) = address {
                config.avatar.eth_address = address;
                update_avatar(&config.avatar.eth_address);
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
