use bevy::prelude::*;

use serde::Deserialize;
use serde::Serialize;

use std::fs::File;
use std::io::BufReader;
use serde_json::Result;

pub fn start() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}


#[derive(Serialize, Deserialize)]
struct Person {
    name: String
}


#[derive(Deserialize, Debug)]
struct Scene {
    name: String,
    entities: Vec<Entitiy>,
}

#[derive(Deserialize, Debug)]
struct Entitiy {
    name: String,
    components: Vec<Component>,
}


#[derive(Deserialize, Debug)]
enum Component{
    Transform(EntityTransform),
    SpriteRenderer(SpriteRenderer),
    CircleCollider(CircleCollider),
    BoxCollider(BoxCollider),
    AlphaCollider(AlphaCollider)
}

#[derive(Deserialize, Debug)]
struct EntityTransform {
    location: Vec2,
    rotation: Vec3,
    scale: Vec2,

}

#[derive(Deserialize, Debug)]
struct SpriteRenderer {
    sprite: String,
    color: Vec4,
    layer: i32,
}

#[derive(Deserialize, Debug)]
struct CircleCollider {
    center: Vec2,
    raius: i32,
}

#[derive(Deserialize, Debug)]
struct BoxCollider {
    center: Vec2,
    size: Vec2,
}

#[derive(Deserialize, Debug)]
struct AlphaCollider {
    sprite: String,
    channel: i32,
}



/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {


    if let Ok(file) = File::open("./assets/scene.json")
    {
        let reader = BufReader::new(file);
        let scene: Result<Scene> = serde_json::from_reader(reader);
        if scene.is_ok()
        {
            let scene = scene.unwrap();

            for entity in  scene.entities.iter()
            {
                let mut transform = Transform::identity();
                let mut texture: Handle<Image> = Handle::default();
                let mut sprite = Sprite::default();
                     
               for component in entity.components.iter()
               {
                
                if let Component::Transform(entity_transform) = component
                {

                    transform.translation = entity_transform.location.extend(0.0);
                    transform.rotation = Quat::from_euler(
                        EulerRot::XYZ,
                        entity_transform.rotation.x,
                        entity_transform.rotation.y,
                        entity_transform.rotation.z);

                    transform.scale = entity_transform.scale.extend(1.0);
                    println!("{:?}", transform);
                }

                if let Component::SpriteRenderer(sprite_renderer) = component
                {
                    sprite.color = Color::Rgba { 
                        red: sprite_renderer.color.x, 
                        green: sprite_renderer.color.y, 
                        blue: sprite_renderer.color.z, 
                        alpha:  sprite_renderer.color.w};
                    texture = asset_server.load(&sprite_renderer.sprite);
                }
               }

               commands.spawn_bundle(SpriteBundle {
                transform,
                sprite: sprite.clone(),
                texture: texture.clone(),
                ..default()
                });
            }
        }

    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

}
