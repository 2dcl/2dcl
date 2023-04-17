//! Plays animations from a skinned glTF.

use std::f32::consts::PI;
use std::time::Duration;
use bevy::gltf::Gltf;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::{
  core_pipeline::clear_color::ClearColorConfig,
  reflect::TypeUuid,
  render::{
      camera::RenderTarget,
      render_resource::{
          AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
          TextureUsages,
      },
      texture::BevyDefault,
      view::RenderLayers,
  },
  sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_capture_media::{MediaCapture, BevyCapturePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugin(bevy_capture_media::BevyCapturePlugin)
        .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
        .insert_resource(State::LoadingGltf)
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(material_update)
        .add_system(state_updater.after(setup_stuff))
        //.add_system(setup_scene_once_loaded)
        .add_system(setup_stuff)
        .add_system(take_screenshot)
        .add_system(keyboard_animation_control)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);


#[derive(Resource)]
enum State{
  LoadingGltf,
  Idle(u8),
  Running(u8),
}

#[derive(Component)]
struct LoadingGLTF(bool);


fn state_updater(
  mut state: ResMut<State>,
  mut capture: MediaCapture,
  mut players: Query<&mut AnimationPlayer>,
  animations: Res<Animations>,
)
{
  match state.as_mut()
  {
    State::LoadingGltf => {},
    State::Idle(frames_passed) => {
      let frames_passed = *frames_passed+1;
      if frames_passed > 20
      {
        *state = State::Running(0);
        for mut player in players.iter_mut()
        {
          player.play(animations.0[1].clone_weak()).repeat();
        }
      } else
      {
        *state = State::Idle(frames_passed);
      }
    },
    State::Running(frames_passed) => {
      let frames_passed = *frames_passed+1;
      if frames_passed > 9
      {
        capture.capture_png(1357);
        *state = State::LoadingGltf;
      } else
      {
        *state = State::Running(frames_passed);
      }
    },
  }
}
// Once the scene is loaded, start the animation
fn setup_stuff(
  mut commands: Commands,
  mut scene: Query<(&Children, &mut LoadingGLTF)>,
  children: Query<(Entity, &Children), Without<LoadingGLTF>>,
  mut players: Query<&mut AnimationPlayer>,
  animations: Res<Animations>,
  mut done: Local<bool>,
  mut state: ResMut<State>
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
         // player.set_speed(0.25);
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

      *state = State::Idle(0);
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





fn setup(
  mut commands: Commands,
  windows: Query<&Window>,
  mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
  mut images: ResMut<Assets<Image>>,
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut capture: MediaCapture,
) {

  // This assumes we only have a single window
  let window = windows.single();

  let size = Extent3d {
      width: window.resolution.physical_width(),
      height: window.resolution.physical_height(),
      ..default()
  };

  // This is the texture that will be rendered to.
  let mut image = Image {
      texture_descriptor: TextureDescriptor {
          label: None,
          size,
          dimension: TextureDimension::D2,
          format: TextureFormat::bevy_default(),
          mip_level_count: 1,
          sample_count: 1,
          usage: TextureUsages::TEXTURE_BINDING
              | TextureUsages::COPY_DST
              | TextureUsages::RENDER_ATTACHMENT,
          view_formats: &[],
      },
      ..default()
  };

  // fill image.data with zeroes
  image.resize(size);

  let image_handle = images.add(image);

  // Main camera, first to render
  commands.spawn((
      Camera3dBundle {
          camera_3d: Camera3d {
              clear_color: ClearColorConfig::Custom(Color::rgba(1., 1., 1., 0.)),
              ..default()
          },
          camera: Camera {
              target: RenderTarget::Image(image_handle.clone()),
              ..default()
          },
          transform: Transform::from_translation(Vec3::new(3., 4.0, 0.))
              .looking_at(Vec3::new(0.,1.5,0.), Vec3::Y),
          ..default()
      },
      // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
      // the cost of rendering the UI without any post processing effects.
      UiCameraConfig { show_ui: false },
  ));

  // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
  let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

  let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
      size.width as f32,
      size.height as f32,
  ))));

  // This material has the texture that has been rendered.
  let material_handle = post_processing_materials.add(PostProcessingMaterial {
      source_image: image_handle,
  });

  // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
  commands.spawn((
      MaterialMesh2dBundle {
          mesh: quad_handle.into(),
          material: material_handle,
          transform: Transform {
              translation: Vec3::new(0.0, 0.0, 1.5),
              ..default()
          },
          ..default()
      },
      post_processing_pass_layer,
  ));

  // The post-processing pass camera.
  let camera_entity =commands.spawn((
      Camera2dBundle {
        camera_2d: Camera2d{
          clear_color: ClearColorConfig::Custom(Color::BLUE)
        },
        camera: Camera {
              // renders after the first main camera which has default value: 0.
              order: 1,
              ..default()
          },
          ..Camera2dBundle::default()
      },
      post_processing_pass_layer,
  )).id();

  capture.start_tracking_camera(1357, camera_entity, Duration::from_secs(5));

  // Insert a resource with the current scene information
  commands.insert_resource(Animations(vec![
      asset_server.load("avatar/idle.glb#Animation0"),
      asset_server.load("avatar/run.glb#Animation0"),
  ]));


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



// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct PostProcessingMaterial {
  /// In this example, this image will be the result of the main camera.
  #[texture(0)]
  #[sampler(1)]
  source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
  fn fragment_shader() -> ShaderRef {
      "PostProcess.wgsl".into()
  }
}

pub fn take_screenshot(
  input: Res<Input<KeyCode>>,
  mut capture: MediaCapture,
) {
  if input.just_released(KeyCode::RShift) {
      // If you have many cameras, consider storing their IDs
      // in a resource
      println!("taking screenshot");
      capture.capture_png(1357);
  }
}