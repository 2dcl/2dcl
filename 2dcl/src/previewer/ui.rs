use bevy::prelude::*;

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

    let mut font_path = std::env::current_exe().unwrap_or_default();
    font_path.pop();
    font_path.push("assets");
    font_path.push("fonts");
    font_path.push("FiraSans-Bold.ttf");

    commands.spawn(TextBundle {
    style: Style {
      size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      ..default()
    },
    text: Text::from_section("Press 'R' to reload the sceen\nHold 'C' for collision view\nPress numbers 1 to 9 to switch levels", 
  TextStyle { font: asset_server.load(font_path), font_size: 20., color: Color::WHITE}),
    ..default()
  }).set_parent(canvas);
}
