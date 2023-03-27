use bevy::prelude::*;

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
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .id();

    let mut asset_path = std::env::current_exe().unwrap_or_default();
    asset_path.pop();
    asset_path.push("assets");

    let mut font_path = asset_path.clone();
    font_path.push("fonts");
    font_path.push("FiraSans-Bold.ttf");

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("background.png");

    let background = commands
        .spawn(ImageBundle {
            image: asset_server.load(image_path).into(),
            background_color: BackgroundColor(Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 0.5,
            }),
            style: Style {
                size: Size::new(Val::Px(260.0), Val::Px(250.0)),
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
        asset_server.load(font_path.as_path()),
        "reload scene",
        background,
    );

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("button_c.png");

    make_ui_tooltip(
        &mut commands,
        vec![asset_server.load(image_path)],
        asset_server.load(font_path.as_path()),
        "show collisions",
        background,
    );

    let mut image_path_1 = asset_path.clone();
    image_path_1.push("ui");
    image_path_1.push("button_0.png");

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
        asset_server.load(font_path.as_path()),
        "change level",
        background,
    );

    let mut image_path = asset_path.clone();
    image_path.push("ui");
    image_path.push("button_u.png");

    make_ui_tooltip(
        &mut commands,
        vec![asset_server.load(image_path)],
        asset_server.load(font_path),
        "toggle ui",
        background,
    );

    fn make_ui_tooltip(
        commands: &mut Commands,
        images: Vec<Handle<Image>>,
        font: Handle<Font>,
        display_text: &str,
        parent: Entity,
    ) {
        let node = commands
            .spawn(NodeBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(20.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    size: Size::new(Val::Percent(100.0), Val::Px(34.0)),
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
                            right: Val::Px(5.),
                            top: Val::Px(5.),
                            bottom: Val::Px(0.),
                        },
                        size: Size::new(Val::Px(32.0), Val::Px(34.0)),
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
                        left: Val::Px(5.),
                        right: Val::Px(0.),
                        top: Val::Px(10.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                text: Text::from_section(
                    display_text,
                    TextStyle {
                        font,
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            })
            .set_parent(node);
    }
}
