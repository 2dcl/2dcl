// use bevy::prelude::Vec4;
// use psd::*;
// use image::*;
// use crate::renderer::scene_loader;
// use crate::renderer::scene_loader::EntityComponent;
// use crate::renderer::scene_loader::SpriteRenderer;
// use super::scene_loader::SceneEntity;
// use super::scene_loader::EntityAnchor;


pub fn psd_read()
{

   /*let psd_bytes = include_bytes!("../../assets/scene/scene.psd");

    let psd;

    match Psd::from_bytes(psd_bytes)
    {
        Ok(v) => psd = v,
        Err(_e) => return
    } 
    
    let layers = psd.layers();

    let mut entities : Vec<SceneEntity> = Vec::new();

    let mut layer_z = layers.len() as i32;

    for layer in layers.iter()
    {

        if layer.blend_mode() as u8 == 1 as u8
        {        
            let pixels = layer.rgba();
            let img:RgbaImage;

            match RgbaImage::from_raw(psd.width(), psd.height(), pixels)
            {
                Some(v) => img = v,
                None => return
            } 
            
            let mut save_path = "./assets/".to_owned();
            save_path = save_path + layer.name() + ".png";

            match img.save(save_path.clone())
            {
                Ok(_v) => println!("saved image {:?}", save_path),
                Err(_e) => println!("couldnt save image{:?}", save_path)
            }
            let mut components: Vec<EntityComponent> = Vec::new();

            let sprite_renderer = EntityComponent::SpriteRenderer{
                0:SpriteRenderer{
                    sprite: layer.name().to_owned() + ".png",
                    color: Vec4::new(1.0,1.0,1.0,1.0),
                    layer:layer_z,
                    flip_x: false,
                    flip_y: false,
                    anchor: EntityAnchor::BottomCenter
                }
            };
            components.push(sprite_renderer);
            entities.push(SceneEntity{name: layer.name().to_owned(),components});
            layer_z-=1;
        }
    }



    //let scene: scene_loader::Scene = scene_loader::Scene{name:"2DCL_psdSceneTest".to_owned(),entities : entities,size_x:1,size_y:1};
    //scene_loader::save_scene(scene, "./assets/scene.json");
 */ 
}
