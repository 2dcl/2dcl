//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containting a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
    //  core_pipeline::clear_color::ClearColorConfig,
      ecs::system::{lifetimeless::SRes, SystemParamItem},
      prelude::*,
      reflect::TypeUuid,
      render::{
          camera::{Camera, RenderTarget},
          render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
          render_resource::{
              BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
              BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
              Extent3d, SamplerBindingType, ShaderStages, TextureDescriptor, TextureDimension,
              TextureFormat, TextureSampleType, TextureUsages, TextureViewDimension,
          },
          renderer::RenderDevice,
          view::RenderLayers,
      },
      sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
  };
  
  use bevy::gltf::Gltf;
  use bevy::input::mouse::*;
  fn main() {
      let mut app = App::new();
      app.add_plugins(DefaultPlugins)
          .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
          .insert_resource(Msaa { samples: 1 })
          .add_startup_system(setup)
          .add_startup_system(load_gltf)
          .add_system(gltf_manual_entity)
          .add_system(remove_colliders)
          .add_system(pan_orbit_camera);
         // .add_system(main_camera_cube_rotator_system);
  
      app.run();
  }
  
  /// Marks the first camera cube (rendered to a texture.)
  #[derive(Component)]
  struct MainCube;
  

  fn setup(
      mut commands: Commands,
   //   mut windows: ResMut<Windows>,
      mut meshes: ResMut<Assets<Mesh>>,
      mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
      mut images: ResMut<Assets<Image>>,
      // asset_server: Res<AssetServer>
      
  ) {
   //   let window = windows.get_primary_mut().unwrap();
      let size = Extent3d {
          width: 512,//window.physical_width(),
          height: 512,//window.physical_height(),
          ..default()
      };
  
      // This is the texture that will be rendered to.
      let mut image = Image {
          texture_descriptor: TextureDescriptor {
              label: None,
              size,
              dimension: TextureDimension::D2,
              format: TextureFormat::Bgra8UnormSrgb,
              mip_level_count: 1,
              sample_count: 1,
              usage: TextureUsages::TEXTURE_BINDING
                  | TextureUsages::COPY_DST
                  | TextureUsages::RENDER_ATTACHMENT,
          },
          ..default()
      };
  
      // fill image.data with zeroes
      image.resize(size);
  
      let image_handle = images.add(image);
  
  
  
  
  /*   commands.spawn_bundle(SceneBundle {
          scene: asset_server.load("whale.glb#Scene0"),
          transform: Transform::from_xyz(-175.0, 0.0, 50.0),
          ..default()
      });
      commands.spawn_bundle(SceneBundle {
          scene: asset_server.load("trading_center.glb#Scene0"),
          transform: Transform::from_xyz(-175.0, 0.0, 50.0),
          ..default()
      });
      commands.spawn_bundle(SceneBundle {
          scene: asset_server.load("agora.glb#Scene0"),
          transform: Transform::from_xyz(-175.0, 0.0, 50.0),
          ..default()
      });
  
      commands.spawn_bundle(SceneBundle {
          scene: asset_server.load("auditorium.glb#Scene0"),
          transform: Transform::from_xyz(-175.0, 0.0, 50.0),
          ..default()
      }); */ 
   /*  let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
      let cube_material_handle = materials.add(StandardMaterial {
          base_color: Color::rgb(0.8, 0.7, 0.6),
          reflectance: 0.02,
          unlit: false,
          ..default()
      });
  
      // The cube that will be rendered to the texture.
      commands
          .spawn_bundle(PbrBundle {
              mesh: cube_handle,
              material: cube_material_handle,
              transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
              ..default()
          })
          .insert(MainCube);
   */ 
      // Light
      // NOTE: Currently lights are ignoring render layers - see https://github.com/bevyengine/bevy/issues/3462
      commands.spawn_bundle(DirectionalLightBundle {
          transform: Transform::identity().looking_at(Vec3::Z*-1.0, Vec3::Y),
          directional_light: DirectionalLight {
               illuminance: 800.0,
               ..default()
          },
          ..default()
      });
  
      commands.insert_resource(AmbientLight {
          color: Color::WHITE,
          brightness: 1.0,
      });
  
      let translation = Vec3::new(160.0,  6.5, -92.47139);
      let radius = translation.length();
  
      let rotation_angle: f32 = -30.0;
  
      let mut orthographic_camera = OrthographicCameraBundle::new_3d();
      orthographic_camera.transform = Transform { 
          translation, 
          rotation: Quat::from_axis_angle(Vec3::X,rotation_angle.to_radians() ), //Quat::from_vec4(Vec4::new(-0.18687499, -0.4096998, -0.086150594, 0.88870704)), 
          scale: Vec3::new(1.0, 1.0, 1.0) };
          orthographic_camera.camera = Camera {
             target: RenderTarget::Image(image_handle.clone()),
              ..default()
          };
          orthographic_camera.orthographic_projection = OrthographicProjection{
              scale: 0.25,
              ..default()
          };
  
      // Main camera, first to render
      commands.spawn_bundle(orthographic_camera).insert(PanOrbitCamera {
          radius,
          ..Default::default()});
  
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
      commands
          .spawn_bundle(MaterialMesh2dBundle {
              mesh: quad_handle.into(),
              material: material_handle,
              transform: Transform {
                  translation: Vec3::new(0.0, 0.0, 1.5),
                  ..default()
              },
              ..default()
          })
          .insert(post_processing_pass_layer);
  
      // The post-processing pass camera.
      commands
         .spawn_bundle(OrthographicCameraBundle::new_2d())
         .insert(post_processing_pass_layer);
  }
  
  
  // Region below declares of the custom material handling post processing effect
  
  /// Our custom post processing material
  #[derive(TypeUuid, Clone)]
  #[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
  struct PostProcessingMaterial {
      /// In this example, this image will be the result of the main camera.
      source_image: Handle<Image>,
  }
  
  struct PostProcessingMaterialGPU {
      bind_group: BindGroup,
  }
  
  impl Material2d for PostProcessingMaterial {
      fn bind_group(material: &PostProcessingMaterialGPU) -> &BindGroup {
          &material.bind_group
      }
  
      fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
          render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
              label: None,
              entries: &[
                  BindGroupLayoutEntry {
                      binding: 0,
                      visibility: ShaderStages::FRAGMENT,
                      ty: BindingType::Texture {
                          multisampled: false,
                          view_dimension: TextureViewDimension::D2,
                          sample_type: TextureSampleType::Float { filterable: true },
                      },
                      count: None,
                  },
                  BindGroupLayoutEntry {
                      binding: 1,
                      visibility: ShaderStages::FRAGMENT,
                      ty: BindingType::Sampler(SamplerBindingType::Filtering),
                      count: None,
                  },
              ],
          })
      }
  
      fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
          asset_server.watch_for_changes().unwrap();
          Some(asset_server.load("PostProcess.wgsl"))
      }
  }
  
  impl RenderAsset for PostProcessingMaterial {
      type ExtractedAsset = PostProcessingMaterial;
      type PreparedAsset = PostProcessingMaterialGPU;
      type Param = (
          SRes<RenderDevice>,
          SRes<Material2dPipeline<PostProcessingMaterial>>,
          SRes<RenderAssets<Image>>,
      );
  
      fn prepare_asset(
          extracted_asset: PostProcessingMaterial,
          (render_device, pipeline, images): &mut SystemParamItem<Self::Param>,
      ) -> Result<PostProcessingMaterialGPU, PrepareAssetError<PostProcessingMaterial>> {
          let (view, sampler) = if let Some(result) = pipeline
              .mesh2d_pipeline
              .get_image_texture(images, &Some(extracted_asset.source_image.clone()))
          {
              result
          } else {
              return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
          };
  
          let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
              label: None,
              layout: &pipeline.material2d_layout,
              entries: &[
                  BindGroupEntry {
                      binding: 0,
                      resource: BindingResource::TextureView(view),
                  },
                  BindGroupEntry {
                      binding: 1,
                      resource: BindingResource::Sampler(sampler),
                  },
              ],
          });
          Ok(PostProcessingMaterialGPU { bind_group })
      }
  
      fn extract_asset(&self) -> PostProcessingMaterial {
          self.clone()
      }
  }
  
  
  struct GltfSpawnCheck {
      spawned: bool
  }
  
  struct MyGLTFScene {
     scene: Handle<Scene>
  }
  
fn CenterBoundigBox(
    //mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
)
{
  let min_limit: Vec3;
  let max_limit: Vec3;

  for mesh in meshes.iter()
  {
    meshes.get(mesh).unwrap().attribute(Mesh::ATTRIBUTE_POSITION);
    
  }

}
  fn remove_colliders(
      mut commands: Commands,
      query: Query<(Entity, &Name)>,
  )
  {
      for (entity,name) in query.iter() {
          if name.contains("collider")
          {
              commands.entity(entity).despawn();
  
          }
      }
      
  }
  fn gltf_manual_entity(
      mut commands: Commands,
      my: Res<MyAssetPack>,
      assets_gltf: Res<Assets<Gltf>>,
      mut assets_scenes: ResMut<Assets<Scene>>,
      //assets_gltfmesh: Res<Assets<GltfMesh>>,
      //assets_gltfnode: Res<Assets<GltfNode>>,
      mut config: ResMut<GltfSpawnCheck>,
  ) {
      if !config.spawned
      {
          
          if  let Some(gltf) = assets_gltf.get(&my.0) {
            /* let mut world = World::new();
  
          let mut MyScene = gltf.scenes[0].clone();
           let mut world = &assets_scenes.get_mut(&MyScene).unwrap().world;
          let mut query = world.query::<(&Entity, &Name)>();
          
          for (entity,name) in query.iter_mut(&mut world) {
              if name.contains("collider")
              {
                  println!("{} ends with collider", name);
              }
          }*/  
          commands.spawn_scene(gltf.scenes[0].clone());
          // Get the GLTF Mesh named "CarWheel"
          // (unwrap safety: we know the GLTF has loaded already)
          // Iterate over everything.
  /* 
           for (name, node) in &gltf.named_nodes {
              
         
              if!name.contains("_collider"){
            //  println!("{} doesnt end with collider", name);
              let current_node = assets_gltfnode.get(node).unwrap();
              if let Some(mesh) = current_node.mesh.clone()
              {
              let current_mesh = assets_gltfmesh.get(mesh).unwrap();
              //let material = current_mesh.primitives[0].material.clone().unwrap();
              if let Some(material) = current_mesh.primitives[0].material.clone()
              {   println!("{} has material", name);
  
                  commands.spawn_bundle(PbrBundle {
                      mesh: current_mesh.primitives[0].mesh.clone(),
                      // (unwrap: material is optional, we assume this primitive has one)
                      material: material,
                      transform: current_node.transform,
                      ..Default::default()
                      });
                      }
          // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
            
              else
              {
                  println!("{} doesnt have material", name);
                  commands.spawn_bundle(PbrBundle {
                      mesh: current_mesh.primitives[0].mesh.clone(),
                      transform: current_node.transform,
                      // (unwrap: material is optional, we assume this primitive has one)
                      ..Default::default()
                      });
                      }
  
              }
              else
              {
                  println!("{} ends with collider", name);
              }
              
          }  }
        */
          println!("Finished spawning"); 
          config.spawned =true;
         
      }
      }
      
  }
  
  struct MyAssetPack(Handle<Gltf>);
  
  fn load_gltf(
      mut commands: Commands,
      ass: Res<AssetServer>,
  ) {
      let gltf = ass.load("core_building.glb");
      commands.insert_resource(MyAssetPack(gltf));
      commands.insert_resource(GltfSpawnCheck {spawned:false});
  }
  
  
  /// Tags an entity as capable of panning and orbiting.
  #[derive(Component)]
  struct PanOrbitCamera {
      /// The "focus point" to orbit around. It is automatically updated when panning the camera
      pub focus: Vec3,
      pub radius: f32,
      pub upside_down: bool,
  }
  
  impl Default for PanOrbitCamera {
      fn default() -> Self {
          PanOrbitCamera {
              focus: Vec3::ZERO,
              radius: 5.0,
              upside_down: false,
          }
      }
  }
  
  /// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
  fn pan_orbit_camera(
      windows: Res<Windows>,
      mut ev_motion: EventReader<MouseMotion>,
      mut ev_scroll: EventReader<MouseWheel>,
      input_mouse: Res<Input<MouseButton>>,
      mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
  ) {
      
      // change input mapping for orbit and panning here
      let orbit_button = MouseButton::Right;
      let pan_button = MouseButton::Middle;
  
      let mut pan = Vec2::ZERO;
      let mut rotation_move = Vec2::ZERO;
      let mut scroll = 0.0;
      let mut orbit_button_changed = false;
  
      if input_mouse.pressed(orbit_button) {
          for ev in ev_motion.iter() {
              rotation_move += ev.delta;
          }
      } else if input_mouse.pressed(pan_button) {
          // Pan only if we're not rotating at the moment
          for ev in ev_motion.iter() {
              pan += ev.delta;
          }
      }
      for ev in ev_scroll.iter() {
          scroll += ev.y;
      }
      if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
          orbit_button_changed = true;
      }
  
      for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
          if orbit_button_changed {
              // only check for upside down when orbiting started or ended this frame
              // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
              let up = transform.rotation * Vec3::Y;
              pan_orbit.upside_down = up.y <= 0.0;
          }
  
          let mut any = false;
          if rotation_move.length_squared() > 0.0 {
              any = true;
              let window = get_primary_window_size(&windows);
              let delta_x = {
                  let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                  if pan_orbit.upside_down { -delta } else { delta }
              };
              let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
              let yaw = Quat::from_rotation_y(-delta_x);
              let pitch = Quat::from_rotation_x(-delta_y);
              transform.rotation = yaw * transform.rotation; // rotate around global y axis
              transform.rotation = transform.rotation * pitch; // rotate around local x axis
          } else if pan.length_squared() > 0.0 {
              any = true;
              // make panning distance independent of resolution and FOV,
              let window = get_primary_window_size(&windows);
              pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
              // translate by local axes
              let right = transform.rotation * Vec3::X * -pan.x;
              let up = transform.rotation * Vec3::Y * pan.y;
              // make panning proportional to distance away from focus point
              let translation = (right + up) * pan_orbit.radius;
              pan_orbit.focus += translation;
          } else if scroll.abs() > 0.0 {
              any = true;
              pan_orbit.radius -= scroll  * 2.0;
          
              // dont allow zoom to reach zero or you get stuck
             // pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
            //  println!("Radius: {}, scroll: {} ", pan_orbit.radius,scroll);
          }
  
          if any {
              // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
              // parent = x and y rotation
              // child = z-offset
              let rot_matrix = Mat3::from_quat(transform.rotation);
              transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
              println!("Camera transform: {:?} ", transform);
          }
      }
  }
  
  fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
      let window = windows.get_primary().unwrap();
      let window = Vec2::new(window.width() as f32, window.height() as f32);
      window
  }