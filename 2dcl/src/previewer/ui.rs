use bevy::prelude::*;

#[derive(Component, Default)]
pub enum Message {
    #[default]
    None,
    AwaitingSignature,
    PreparingFiles,
    UploadingFiles,
    Success(Timer),
    Error(String),
}

pub fn toggle_ui(keyboard: Res<Input<KeyCode>>, mut ui_query: Query<&mut Visibility, With<Node>>) {
    if keyboard.just_pressed(KeyCode::U) {
        for mut ui_visibility in ui_query.iter_mut() {
            *ui_visibility = match ui_visibility.as_ref() {
                Visibility::Inherited => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Visible => Visibility::Hidden,
            }
        }
    }
}

pub fn update_messages(mut messages_query: Query<(&mut Text, &mut Message)>, time: Res<Time>) {
    for (mut text, mut message) in messages_query.iter_mut() {
        text.sections[0].value = match message.as_mut() {
            Message::None => String::default(),
            Message::AwaitingSignature => {
                format!("Awaiting signature{}", make_elipsis(&time))
            }
            Message::PreparingFiles => {
                format!("Preparing files{}", make_elipsis(&time))
            }
            Message::UploadingFiles => {
                format!("Uploading files{}", make_elipsis(&time))
            }
            Message::Success(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *message = Message::None;
                    "".to_string()
                } else {
                    "Scene deployed successfully".to_string()
                }
            }
            Message::Error(err) => err.clone(),
        }
    }

    fn make_elipsis(time: &Time) -> String {
        let total_elipsis = time.elapsed().as_secs() % 3;
        let mut elipsis = ".".to_string();
        for _ in 0..total_elipsis {
            elipsis += ".";
        }
        elipsis
    }
}
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut asset_path = std::env::current_exe().unwrap_or_default();
    asset_path.pop();
    asset_path.push("assets");

    let mut font_path = asset_path.clone();
    font_path.push("fonts");
    font_path.push("kongtext.ttf");

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("background.png");

    let font: Handle<Font> = asset_server.load(font_path);

    let canvas = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .id();

    let background = commands
        .spawn(ImageBundle {
            image: asset_server.load(image_path).into(),
            background_color: BackgroundColor(Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 0.75,
            }),
            style: Style {
                width: Val::Px(180.0),
                height: Val::Px(160.0),
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,

                ..default()
            },
            ..default()
        })
        .set_parent(canvas)
        .id();

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("button_r.png");

    make_ui_tooltip(
        &mut commands,
        vec![asset_server.load(image_path)],
        &font,
        "reload scene",
        background,
    );

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("button_c.png");

    make_ui_tooltip(
        &mut commands,
        vec![asset_server.load(image_path)],
        &font,
        "show collisions",
        background,
    );

    let mut image_path_1 = asset_path.clone();
    image_path_1.push("ui");
    image_path_1.push("button_1.png");

    let mut image_path_2 = asset_path.clone();
    image_path_2.push("ui");
    image_path_2.push("ellipsis.png");

    let mut image_path_3 = asset_path.clone();
    image_path_3.push("ui");
    image_path_3.push("button_9.png");
    make_ui_tooltip(
        &mut commands,
        vec![
            asset_server.load(image_path_1),
            asset_server.load(image_path_2),
            asset_server.load(image_path_3),
        ],
        &font,
        "change level",
        background,
    );

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("button_u.png");

    make_ui_tooltip(
        &mut commands,
        vec![asset_server.load(image_path)],
        &font,
        "toggle ui",
        background,
    );

    let mut image_path_1 = asset_path.clone();
    image_path_1.push("ui");
    image_path_1.push("button_ctrl.png");

    let mut image_path_2 = asset_path.clone();
    image_path_2.push("ui");
    image_path_2.push("plus_sign.png");

    let mut image_path_3 = asset_path.clone();
    image_path_3.push("ui");
    image_path_3.push("button_p.png");

    make_ui_tooltip(
        &mut commands,
        vec![
            asset_server.load(image_path_1),
            asset_server.load(image_path_2),
            asset_server.load(image_path_3),
        ],
        &font,
        "deploy",
        background,
    );

    make_messages_ui(&mut commands, &font, canvas);

    fn make_ui_tooltip(
        commands: &mut Commands,
        images: Vec<Handle<Image>>,
        font: &Handle<Font>,
        display_text: &str,
        parent: Entity,
    ) {
        let node = commands
            .spawn(NodeBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(15.),
                        right: Val::Px(0.),
                        top: Val::Px(9.5),
                        bottom: Val::Px(0.),
                    },
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            })
            .set_parent(parent)
            .id();

        for image in images {
            commands
                .spawn(ImageBundle {
                    image: image.into(),
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(2.),
                            top: Val::Px(2.),
                            bottom: Val::Px(0.),
                        },
                        width: Val::Px(16.0),
                        height: Val::Px(17.0),
                        ..default()
                    },
                    ..default()
                })
                .set_parent(node);
        }

        commands
            .spawn(TextBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(6.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                text: Text::from_section(
                    display_text,
                    TextStyle {
                        font: font.clone(),
                        font_size: 8.,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            })
            .set_parent(node);
    }

    fn make_messages_ui(commands: &mut Commands, font: &Handle<Font>, parent: Entity) {
        let node = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            })
            .set_parent(parent)
            .id();

        commands
            .spawn((
                TextBundle {
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(10.),
                            top: Val::Px(0.),
                            bottom: Val::Px(10.),
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                Message::default(),
            ))
            .set_parent(node);
    }
}
