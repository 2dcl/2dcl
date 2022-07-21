
use bevy::prelude::*;
mod Capture;
use Capture::CapturePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(CapturePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_gltf)
        .run();
}


fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("whale.glb#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    commands.spawn_bundle(TransformBundle {
        local: Transform::identity(),
        global: GlobalTransform::identity(),
    }).with_children(|parent| {
        parent.spawn_scene(my_gltf);
    });

    commands.spawn_bundle(TransformBundle {
        local: Transform::identity(),
        global: GlobalTransform::identity(),
    }).with_children(|parent| {
        parent.spawn_scene(ass.load("moon-tower.glb#Scene0"));
    });

    commands.spawn_bundle(TransformBundle {
        local: Transform::identity(),
        global: GlobalTransform::identity(),
    }).with_children(|parent| {
        parent.spawn_scene(ass.load("mountains.glb#Scene0"));
    });

}
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
/*    // plane
   commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 50.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(200.0, 200.0, 0.0),
        ..default()
    });*/
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        ..default()
    });
}
