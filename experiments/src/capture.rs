use bevy::core_pipeline::{
    draw_3d_graph, node, AlphaMask3d, Opaque3d, RenderTargetClearColors, Transparent3d,
};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::{Camera,ActiveCamera, CameraTypePlugin, RenderTarget};
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssets};
use bevy::render::render_graph::{NodeRunError, RenderGraph, RenderGraphContext, SlotValue};
use bevy::render::render_phase::RenderPhase;
use bevy::render::render_resource::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, ImageCopyBuffer,
    ImageDataLayout, MapMode, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, SamplerBindingType, ShaderStages, 
    TextureSampleType, TextureViewDimension
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::{RenderApp, RenderStage};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle};
use bevy::ecs::system::{lifetimeless::SRes, SystemParamItem};
use bevy::gltf::Gltf;

#[derive(Component, Default)]
pub struct CaptureCamera;

pub const CAPTURE_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 13373934772014884929);

// The name of the final node of the first pass.
pub const CAPTURE_DRIVER: &str = "capture_driver";

pub struct GltfSpawnCheck {
    spawned: bool,
    taken_screenshot: bool,
    timer : Timer,
}
/// Helper resource for tracking our asset
pub struct MyAssetPack(Handle<Gltf>);

fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut config: ResMut<GltfSpawnCheck>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // spawn the first scene in the file
        commands.spawn_scene(gltf.scenes[0].clone());
        config.spawned = true;
    }
}

fn load_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {

   commands.insert_resource(MyAssetPack(ass.load("moon-tower.glb")));
   //commands.insert_resource(MyAssetPack(ass.load("whale.glb")));
}

pub fn setup_capture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut clear_colors: ResMut<RenderTargetClearColors>,
    render_device: Res<RenderDevice>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    commands.insert_resource(GltfSpawnCheck {spawned: false, taken_screenshot: false,timer: Timer::from_seconds(5.0, false)});
     
    //LIGHTS
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

    let size = Extent3d {
        width: 512,
        height: 512,
        ..Default::default()
    };

    // This is the texture that will be rendered to.
    let mut png_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..Default::default()
    };
    png_image.resize(size);

    let postprocess_image = png_image.clone();

    let png_image_handle = images.set(CAPTURE_IMAGE_HANDLE, png_image);

    let postprocess_image_handle = images.set(CAPTURE_IMAGE_HANDLE, postprocess_image);
    
    let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(512) * 4;

    


    let render_target = RenderTarget::Image(png_image_handle);

    
        
    // Main camera, first to render
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(50.0, 0.0, 0.0),
        camera: Camera {
            target: RenderTarget::Image(postprocess_image_handle.clone()),
            ..default()
        },
        ..default()
    });



     // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
     let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

     let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
         size.width as f32,
         size.height as f32,
     ))));
 
     // This material has the texture that has been rendered.
     let material_handle = post_processing_materials.add(PostProcessingMaterial {
         source_image: postprocess_image_handle,
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
 
    let size = padded_bytes_per_row as u64 * 512;

    let output_cpu_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("Output Buffer"),
        size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    clear_colors.insert(render_target.clone(), Color::rgba(0.0, 0.0, 0.0,0.0));
    commands
        .spawn_bundle(PerspectiveCameraBundle::<CaptureCamera> {
            transform: Transform::from_xyz(0.0, 500.0, 0.0).looking_at(Vec3::ZERO,  Vec3::Z),
            camera: Camera {
                target: render_target,
                ..default()
            },
            ..PerspectiveCameraBundle::new()
        })
        .insert(Capture {
            buf: output_cpu_buffer,
        }).insert(post_processing_pass_layer);
}

// Add 3D render phases for CAPTURE_CAMERA.
pub fn extract_camera_phases(
    mut commands: Commands,
    cap: Query<&Capture>,
    active: Res<ActiveCamera<CaptureCamera>>,
) {
    if let Some(entity) = active.get() {
        if let Some(cap) = cap.iter().next() {
            commands
                .get_or_spawn(entity)
                .insert_bundle((
                    RenderPhase::<Opaque3d>::default(),
                    RenderPhase::<AlphaMask3d>::default(),
                    RenderPhase::<Transparent3d>::default(),
                ))
                .insert(Capture {
                    buf: cap.buf.clone(),
                });
        }
    }
}

// A node for the first pass camera that runs draw_3d_graph with this camera.
pub struct CaptureCameraDriver {
    pub buf: Option<Buffer>,
}

impl bevy::render::render_graph::Node for CaptureCameraDriver {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();

        if let Some(camera_3d) = world.resource::<ActiveCamera<CaptureCamera>>().get() {
            graph.run_sub_graph(draw_3d_graph::NAME, vec![SlotValue::Entity(camera_3d)])?;

            let gpu_image = gpu_images.get(&CAPTURE_IMAGE_HANDLE.typed()).unwrap();
            let mut encoder = render_context
                .render_device
                .create_command_encoder(&CommandEncoderDescriptor::default());
            let padded_bytes_per_row =
                RenderDevice::align_copy_bytes_per_row((gpu_image.size.width) as usize) * 4;

            let texture_extent = Extent3d {
                width: gpu_image.size.width as u32,
                height: gpu_image.size.height as u32,
                depth_or_array_layers: 1,
            };

            if let Some(buf) = &self.buf {
                encoder.copy_texture_to_buffer(
                    gpu_image.texture.as_image_copy(),
                    ImageCopyBuffer {
                        buffer: buf,
                        layout: ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(
                                std::num::NonZeroU32::new(padded_bytes_per_row as u32).unwrap(),
                            ),
                            rows_per_image: None,
                        },
                    },
                    texture_extent,
                );
                let render_queue = world.get_resource::<RenderQueue>().unwrap();
                render_queue.submit(std::iter::once(encoder.finish()));
            }
        }

        Ok(())
    }
    fn update(&mut self, world: &mut World) {
        for cap in world.query::<&mut Capture>().iter_mut(world) {
            self.buf = Some(cap.buf.clone());
        }
    }
}

pub fn save_img(cap: Query<&Capture>, 
    render_device: Res<RenderDevice>,
    mut config: ResMut<GltfSpawnCheck>,
    time: Res<Time>,
) {
    if config.spawned && !config.taken_screenshot{
    
    if config.timer.just_finished()
    {
   
    if let Some(cap) = cap.iter().next() {
        let large_buffer_slice = cap.buf.slice(..);
        render_device.map_buffer(&large_buffer_slice, MapMode::Read);
        {
            let large_padded_buffer = large_buffer_slice.get_mapped_range();

            image::save_buffer(
                "test.png",
                &large_padded_buffer,
                512,
                512,
                image::ColorType::Rgba8,
            )
            .unwrap();
        }
        cap.buf.unmap();
        config.taken_screenshot = true;
        //exit.send(AppExit);
        }
        
    }
        config.timer.tick(time.delta());

    }
}

#[derive(Component)]
pub struct Capture {
    pub buf: Buffer,
}

pub struct CapturePlugin;
impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        
        app.add_plugin(CameraTypePlugin::<CaptureCamera>::default())
            .add_startup_system(setup_capture)
            .add_startup_system(load_gltf)
            .add_system(spawn_gltf_objects)
            .add_system(save_img);

        let render_app = app.sub_app_mut(RenderApp);

        // This will add 3D render phases for the capture camera.
        render_app.add_system_to_stage(RenderStage::Extract, extract_camera_phases);

        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

        // Add a node for the capture.
        graph.add_node(CAPTURE_DRIVER, CaptureCameraDriver { buf: None });

        // The capture's dependencies include those of the main pass.
        graph
            .add_node_edge(node::MAIN_PASS_DEPENDENCIES, CAPTURE_DRIVER)
            .unwrap();

        // Insert the capture node: CLEAR_PASS_DRIVER -> CAPTURE_DRIVER -> MAIN_PASS_DRIVER
        graph
            .add_node_edge(node::CLEAR_PASS_DRIVER, CAPTURE_DRIVER)
            .unwrap();
        graph
            .add_node_edge(CAPTURE_DRIVER, node::MAIN_PASS_DRIVER)
            .unwrap();
    }
}

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
    /// In this example, this image will be the result of the main camera.
    source_image: Handle<Image>,
}

pub struct PostProcessingMaterialGPU {
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

