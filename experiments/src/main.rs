//! Plays animations from a skinned glTF.

use std::f32::consts::PI;
use std::time::Duration;
use bevy::gltf::Gltf;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(material_update)
        //.add_system(setup_scene_once_loaded)
        .add_system(setup_stuff)
        .add_system(keyboard_animation_control)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);


#[derive(Component)]
struct LoadingGLTF(bool);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("avatar/Attack_emote_v3.glb#Animation0"),
    ]));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 7.5)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Fox


    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/Festival_hat_02.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/dc_halloween_bat.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/Hair_ShortHair_01.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/M_Beard.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/xmas_2021_santa_xray.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/shoes.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/M_uBody_FWShirt.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));
    commands.spawn(SceneBundle {
      scene: asset_server.load("avatar/M_lBody_FWPants.glb#Scene0"),
      ..default()
    }).insert(LoadingGLTF(false));

    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - return: change animation");
}


// Once the scene is loaded, start the animation
fn setup_stuff(
  mut commands: Commands,
  mut scene: Query<(&Children, &mut LoadingGLTF)>,
  children: Query<(Entity, &Children), Without<LoadingGLTF>>,
  mut players: Query<&mut AnimationPlayer>,
  animations: Res<Animations>,
  mut done: Local<bool>,
) {

  if !*done{
    let mut loading_count = 0;
    for (scene_children, mut loading) in scene.iter_mut() {

      if loading.0 == true
      {
        loading_count+=1;
        continue;
      }
      loading.0 = true;

      let child = scene_children.iter().next().unwrap();
      for (other_entity, other_entity_children) in children.iter()
      {
        if child == &other_entity
        {
          let armature = other_entity_children.iter().next().unwrap();
          let mut player =AnimationPlayer::default();
          player.play(animations.0[0].clone_weak()).repeat();
          player.pause();
          commands.entity(*armature).insert(player);
          break;
        }
      }
    }

    if loading_count>=8
    {
      println!("all loaded");
      for mut player in players.iter_mut()
      {
        player.resume();
      }
      *done = true;
    }
  }
}

fn material_update (
  assets_gltf: Res<Assets<Gltf>>,
  mut assets_mats: ResMut<Assets<StandardMaterial>>,
)
{
  for (_, gltf) in assets_gltf.iter()
  {
    for (material_name, material) in &gltf.named_materials
    {
      if material_name.starts_with("AvatarSkin")
      {
        assets_mats.get_mut(material).unwrap().base_color = Color::rgba(0.94921875, 0.76171875, 0.6484375, 1.);
      }
      else if material_name.starts_with("Hair_MAT")
      {
        assets_mats.get_mut(material).unwrap().base_color = Color::rgba(0.98046875, 0.82421875, 0.5078125, 1.);
      }
    }
  }

}
// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }
    }
}