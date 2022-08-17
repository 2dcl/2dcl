use bevy::prelude::Vec4;
use psd::*;
use image::*;
use crate::renderer::scene_deserializer;
use crate::renderer::scene_deserializer::EntityComponent;
use crate::renderer::scene_deserializer::SpriteRenderer;
use super::scene_deserializer::Entitiy;

pub fn psd_read()
{

    let psd_bytes = include_bytes!("../../assets/2DCL_psdSceneTest.psd");

    let psd = Psd::from_bytes(psd_bytes).unwrap();

    let layers = psd.layers();

    let mut entities : Vec<Entitiy> = Vec::new();

    let mut layer_z = layers.len() as i32;

    for layer in layers.iter()
    {

        if layer.blend_mode() as u8 == 1 as u8
        {        
            let pixels = layer.rgba();
            let img:RgbaImage = RgbaImage::from_raw(psd.width(), psd.height(), pixels).unwrap();


            let mut save_path = "./assets/".to_owned();
            save_path = save_path + layer.name() + ".png";

            img.save(save_path);
            let mut components: Vec<EntityComponent> = Vec::new();

            let sprite_renderer = EntityComponent::SpriteRenderer{
                0:SpriteRenderer{
                    sprite: layer.name().to_owned() + ".png",
                    color: Vec4::new(1.0,1.0,1.0,1.0),
                    layer:layer_z,

                }
            };
            components.push(sprite_renderer);
            entities.push(Entitiy{name: layer.name().to_owned(),components});
            layer_z-=1;
        }
    }



    let scene: scene_deserializer::Scene = scene_deserializer::Scene{name:"2DCL_psdSceneTest".to_owned(),entities : entities};
    scene_deserializer::save_scene(scene, "./assets/scene.json");

}