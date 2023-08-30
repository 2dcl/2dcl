use bevy::prelude::*;

use crate::states::AppState;

const DEFAULT_BUTTON: &str = "ui/B_1.png";
const HOVERED_BUTTON: &str = "ui/B_3.png";
const BG_COLOR: Color = Color::rgb(0.098, 0.075, 0.102);
const TEXT_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct DiscoverButton {
    default: Handle<Image>,
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
    let discover_button = DiscoverButton {
        default: asset_server.load(DEFAULT_BUTTON),
        hovered: asset_server.load(HOVERED_BUTTON),
    };

     commands.spawn((ButtonBundle{
        style: Style{
            width: Val::Px(138.),
            height: Val::Px(50.),
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        image: UiImage::new(discover_button.default.clone()),
        ..Default::default()
    },discover_button));

  /*let bg =  commands.spawn(NodeBundle{
        background_color: BackgroundColor(BG_COLOR),
        style: Style{
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Percent(5.)),
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            ..Default::default()
        },
        ..Default::default()
    }).id();
    commands.spawn(TextBundle{
        text: Text::from_section("Scene  |  Parcel  |  Last Update\n\nSample Scene modified  |  -25 , 7  |  Fri, 25 Aug 2023 19:09:58 +0000",TextStyle {
            font: asset_server.load("fonts/Arcadepix Plus.ttf"),
            font_size: 16.0,
            color: TEXT_COLOR,
        }),
        style: Style{
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        ..Default::default()
    }).set_parent(bg);*/
    
}

fn spawn_scene_entry(
    commands: &mut Commands,
    parent: &Entity,
    scene: &SceneDiscoveryData
)
{
    
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut Style, &mut UiImage, &DiscoverButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut style, mut image, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                style.display = Display::None;
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

