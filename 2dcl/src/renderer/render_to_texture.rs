use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};


use image::*;
pub struct RenderToTexturePlugin;

impl Plugin for RenderToTexturePlugin
{

    fn build(&self, app: &mut App) {
    app
    .add_startup_system(setup)
    .add_system(cube_rotator_system)
    .add_system(save_img)
    .add_system(rotator_system);
    }
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

struct RenderOutput(Handle<Image>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
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

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    // The cube that will be rendered to the texture.
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(FirstPassCube)
        .insert(first_pass_layer);

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLUE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                priority: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(first_pass_layer);

    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

   //  Main pass cube, with material containing the rendered first pass texture.
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: material_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..default()
            },
            ..default()
        })
        .insert(MainPassCube);
 
    // The main pass camera.
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });

    commands.insert_resource(RenderOutput(image_handle.clone()));
    commands.insert_resource(PhotoTimer 
        {
            timer: Timer::from_seconds(5.0, false), 
            material_handle: material_handle
        }
    );

}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds());
        transform.rotate_y(0.7 * time.delta_seconds());
    }
}

pub struct PhotoTimer {
    timer : Timer,
    material_handle: Handle<StandardMaterial>,
}

fn save_img(
    render_output: Res<RenderOutput>,
    images: Res<Assets<Image>>,
    time: Res<Time>,
    materials: Res<Assets<StandardMaterial>>,
    mut timer: ResMut<PhotoTimer>,) {

    if timer.timer.just_finished(){
    let image_handle = render_output.0.clone();
    let bytes = images.get(&image_handle).unwrap().data.clone();
    

    //println!("{:?}",images.get(&image_handle).unwrap().texture_descriptor.format);

    //let img2 = ImageReader::open("E:/bevy_shaders/shaders-bevy/test.png").unwrap().decode().unwrap();
   // let img2 = ImageReader::n
   let mut img = RgbaImage::new(512, 512);


   //println!("Complete buffer: {:?}",bytes.clone());
   for x in 0..512
    {
        for y in 0..512
        {
            let b = bytes[x as usize * 4 + y as usize * 10];
            let g = bytes[x as usize * 4 + y as usize * 10 + 1];
            let r = bytes[x as usize * 4 + y as usize * 10 + 2];
            let a = bytes[x as usize * 4 + y as usize * 10 + 3];
            
            if b>0 || g>0 || r>0 || a>0
            {
                println!("x: {:?} y: {:?}:  {:?}",x,y,Rgba([r,g,b,a]));
            }
            
            img.put_pixel(x,y,Rgba([r,g,b,a]));  
        }

    } 
    println!("Saving");
    img.save("E:/test.png");
    //let mut img2 = ImageReader::new(Cursor::new(bytes));
    //img2.set_format(ImageFormat::Png);
    //img2.decode().unwrap();
    //img2.
    //img2.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);
    //img2.save("E:/test.png");
    //let img = ImageBuffer::from_vec(512, 512, bytes).unwrap();
    //img.save("E:/bevy_shaders/shaders-bevy/test.png");
    //let img = ImageBuffer::from_raw(512,512,bytes);
    }

    timer.timer.tick(time.delta());
}