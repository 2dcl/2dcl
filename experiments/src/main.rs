//! Plays animations from a skinned glTF.

use std::f32::consts::PI;
use std::path::{PathBuf, Path};
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
use bevy_capture_media::MediaCapture;
use bevy::window::WindowResolution;
use glob::glob;
use catalyst::{entity_files::SceneFile, ContentClient};

use serde::{Serialize, Deserialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
          primary_window: Some(Window {
            resolution: WindowResolution::new(
              640., 360.
            ),
            ..default()
          }),
          ..default()
        }))
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
        .run();
}


#[derive(Serialize, Deserialize, Debug)]
struct CatalystColor {
  r: f32,
  g: f32,
  b: f32
}
#[derive(Serialize, Deserialize, Debug)]
struct ColoredAvatarPart{
  color: CatalystColor
}

#[derive(Serialize)]
struct CatalystId {
    ids:  Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AvatarSnapshots{
  face: Option<String>,
  face128: Option<String>,
  face256: Option<String>,
  body: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
struct Avatar{
  bodyShape: String,
  snapshots: AvatarSnapshots,
  eyes: ColoredAvatarPart,
  hair: ColoredAvatarPart,
  skin: ColoredAvatarPart,
  wearables: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AvatarList{
  avatars: AvatarList2
}


#[derive(Serialize, Deserialize, Debug)]
struct AvatarList2{
  avatars: AvatarList3
}

#[derive(Serialize, Deserialize, Debug)]
struct AvatarList3{
  avatar: AvatarInfo
}


#[derive(Serialize, Deserialize, Debug)]
struct AvatarInfo{
  userId: String,
  email: String,
  name: String,
  hasClaimedName: bool,
  description: String,
  ethAddress: String,
  version: i16,
  avatar: Avatar,
  tutorialStep: i32,
  interests: Vec<String>,
  unclaimedName: String
}


#[derive(Serialize, Deserialize, Debug)]
struct Request{
  pointers: Vec<String>
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);


#[derive(Resource)]
struct AvatarProperties{
  eth_address: String,
  body_shape: BodyShape,
  eyes_color: CatalystColor,
  hair_color: CatalystColor,
  skin_color: CatalystColor,
  glb_loading_count: usize,
}

enum BodyShape{
  Male,
  Female
}
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
          player.set_speed(4.);
          player.pause();
          player.set_elapsed(0.);
        }
      } else
      {
        for mut player in players.iter_mut()
        {
          let elapsed  = frames_passed as f32 * &player.speed() * 1./60.;
          player.set_elapsed(elapsed);
        }
        *state = State::Idle(frames_passed);
      }
    },
    State::Running(frames_passed) => {
      let frames_passed = *frames_passed+1;
      if frames_passed > 11
      {
        capture.capture_png(1357);
        *state = State::LoadingGltf;
      } else
      {
        for mut player in players.iter_mut()
        {
          let elapsed  = frames_passed as f32 * &player.speed() * 1./60.;
          player.set_elapsed(elapsed);
        }
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
  mut state: ResMut<State>,
  avatar_properties: Res<AvatarProperties>
) {

  if !*done{
    let mut loading_count = 0;
    for (scene_children, mut loading) in scene.iter_mut() {
      
      loading_count+=1;

      if loading.0 == true
      {
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
          player.set_speed(16.);
          commands.entity(*armature).insert(player);
          break;
        }
      }
    }

    if loading_count>=avatar_properties.glb_loading_count
    { 
      *done = true;
      *state = State::Idle(0);
    } 
  }
}

fn material_update (
  assets_gltf: Res<Assets<Gltf>>,
  mut assets_mats: ResMut<Assets<StandardMaterial>>,
  avatar_properties: Res<AvatarProperties>
)
{
  for (_, gltf) in assets_gltf.iter()
  {
    for (material_name, material) in &gltf.named_materials
    {
      if material_name.starts_with("AvatarSkin")
      {
        assets_mats.get_mut(material).unwrap().base_color = Color::rgba(
          avatar_properties.skin_color.r, 
          avatar_properties.skin_color.g, 
          avatar_properties.skin_color.b,
           1.);
      }
      else if material_name.starts_with("Hair_MAT")
      {
        assets_mats.get_mut(material).unwrap().base_color = Color::rgba(
          avatar_properties.hair_color.r, 
          avatar_properties.hair_color.g, 
          avatar_properties.hair_color.b,
           1.);
      }
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

  let mut avatar_properties = download_avatar("0x5e5d9d1dfd87e9b8b069b8e5d708db92be5ade99").unwrap();

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

  let mut base_path =  std::env::current_exe().unwrap();
  base_path.pop();
  let pattern = format!("{}/assets/avatar/{}/**/*.glb",  
  base_path.to_str().unwrap(),
  avatar_properties.eth_address);


  let mut loading_count = 0;
 for entry in glob(pattern.as_str())
      .expect("Failed to read glob pattern")
     
  { 
    let mut entry = entry.unwrap();
    let mut base_dir = std::env::current_exe().unwrap();
    let mut rev_entry = PathBuf::new();
    while entry.parent().is_some()
    {
      rev_entry.push(entry.file_name().unwrap());
      entry.pop();
    }

    while base_dir.pop()
    {
      rev_entry.pop();
    }

    let mut entry = PathBuf::new();
    while rev_entry.parent().is_some()
    {
      entry.push(rev_entry.file_name().unwrap());
      rev_entry.pop();
    }

    match avatar_properties.body_shape{
        BodyShape::Male =>  
        if !entry.to_str().unwrap().to_lowercase().contains("female") && 
          !entry.to_str().unwrap().to_lowercase().contains("/f_") &&
          !entry.to_str().unwrap().to_lowercase().contains("\\f_"){

          commands.spawn(SceneBundle {
            scene: asset_server.load(format!("{}#Scene0",entry.to_str().unwrap())),
            ..default()
          }).insert(LoadingGLTF(false));
       
          loading_count+=1;
        },
        BodyShape::Female =>  
        if !(entry.to_str().unwrap().to_lowercase().contains("male") ||
              entry.to_str().unwrap().to_lowercase().contains("/m_") ||
              entry.to_str().unwrap().to_lowercase().contains("\\m_")) ||
            entry.to_str().unwrap().to_lowercase().contains("female")  {
          
          commands.spawn(SceneBundle {
            scene: asset_server.load(format!("{}#Scene0",entry.to_str().unwrap())),
            ..default()
          }).insert(LoadingGLTF(false));
          loading_count+=1;
        },
    }
  }

  avatar_properties.glb_loading_count = loading_count;
  commands.insert_resource(avatar_properties);
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

#[tokio::main]
async fn download_avatar(eth_address: &str) -> dcl_common::Result<AvatarProperties> {

    let server = catalyst::Server::production();
    let ids= vec![eth_address.to_string()];
    
    let catalyst_id = CatalystId{ids};
    let avatar_list: AvatarList = server.post("/lambdas/profiles", &catalyst_id).await?;

    for urn in avatar_list.avatars.avatars.avatar.avatar.wearables
    {
      let request = Request{pointers: vec![urn.to_string()]};
      let result: Vec<SceneFile> = server.post("/content/entities/active", &request).await?;
      for scene_file in result {

        for downloadable in scene_file.content {

          let mut download_path = std::env::current_exe().unwrap();
          download_path.pop();
          download_path.push("assets");
          download_path.push("avatar");
          download_path.push(eth_address);
          download_path.push(scene_file.id.to_string());
          download_path.push(downloadable.filename.to_str().unwrap());
            ContentClient::download(&server, downloadable.cid, &download_path).await?;
        }

      }  
    }

    let body_shape = match avatar_list.avatars.avatars.avatar.avatar.bodyShape.contains("Female")
    {
      true => BodyShape::Female,
      false => BodyShape::Male,
    };

    let new_avatar = AvatarProperties{
      eth_address: eth_address.to_string(),
      body_shape,
      eyes_color: avatar_list.avatars.avatars.avatar.avatar.eyes.color,
      hair_color: avatar_list.avatars.avatars.avatar.avatar.hair.color,
      skin_color:  avatar_list.avatars.avatars.avatar.avatar.skin.color,
      glb_loading_count: 0,
    };

    Ok(new_avatar)
}
