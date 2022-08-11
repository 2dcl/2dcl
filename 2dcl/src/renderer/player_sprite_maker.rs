use aseprite::SpritesheetData;
use bevy_inspector_egui::egui::Rgba;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer};
use std::fs::File;
use std::path::Path;
use bevy::sprite::Rect;
use bevy::utils::Duration;
use std::path::PathBuf;
use std::io::Error;
use std::fs;

pub fn make_player_spritesheet <P>(
    player_path: P,
    wearables_path: P,

)
where
    P: AsRef<Path> + Clone
{


    let player_file: File = File::open(player_path.clone()).unwrap();
    let player_spritesheet: aseprite::SpritesheetData =  serde_json::from_reader(player_file).unwrap();
   
    let mut player_image_path = PathBuf::new();    
    player_image_path.push(player_path);
    player_image_path.pop();
    player_image_path.push(&player_spritesheet.meta.image.clone().unwrap_or_default());
    let mut player_image = ImageReader::open(player_image_path).unwrap().decode().unwrap();
    if let  DynamicImage::ImageRgba8(mut player_dynamic_image) = player_image.clone()
    {
        let mut wearables_dir = fs::read_dir(wearables_path.clone()).unwrap();
        for wearable_path in wearables_dir
        {
            let wearable_path_string =   wearable_path.unwrap().path().display().to_string();
        
            if wearable_path_string.ends_with("feet.json")
            {  
                add_wearable(&mut player_dynamic_image,wearable_path_string,&player_spritesheet);
            }
        }

        wearables_dir = fs::read_dir(wearables_path.clone()).unwrap();
        for wearable_path in wearables_dir 
        {
            let wearable_path_string =   wearable_path.unwrap().path().display().to_string();
        
            if wearable_path_string.ends_with("lower.json")
            {  
                add_wearable(&mut player_dynamic_image,wearable_path_string,&player_spritesheet);
            }
        }

        wearables_dir = fs::read_dir(wearables_path.clone()).unwrap();
        for wearable_path in wearables_dir 
        {
            let wearable_path_string =   wearable_path.unwrap().path().display().to_string();
        
            if wearable_path_string.ends_with("upper.json")
            {  
                add_wearable(&mut player_dynamic_image,wearable_path_string,&player_spritesheet);
            }
        }
        wearables_dir = fs::read_dir(wearables_path.clone()).unwrap();
        for wearable_path in wearables_dir 
        {
            let wearable_path_string =   wearable_path.unwrap().path().display().to_string();
        
            if wearable_path_string.ends_with("head_accesory.json")
            {  
                add_wearable(&mut player_dynamic_image,wearable_path_string,&player_spritesheet);
            }
        }

        wearables_dir = fs::read_dir(wearables_path.clone()).unwrap();
        for wearable_path in wearables_dir 
        {
            let wearable_path_string =   wearable_path.unwrap().path().display().to_string();
        
            if wearable_path_string.ends_with("hair.json")
            {  
                add_wearable(&mut player_dynamic_image,wearable_path_string,&player_spritesheet);
            }
        }
        println!("saving image");
        player_dynamic_image.save("E:/test.png");
    }
}


fn add_wearable <P> (base_dynamic_image:  &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,  wearable_path:P, base_spritesheet: &SpritesheetData)
where
    P: AsRef<Path> + Clone
{

        let wearable_file: File = File::open(wearable_path.clone()).unwrap();

        let wearable_spritesheet: aseprite::SpritesheetData =  serde_json::from_reader(wearable_file).unwrap();

        let mut wearable_image_path = PathBuf::new();    
        wearable_image_path.push(wearable_path);
        wearable_image_path.pop();
        wearable_image_path.push(&wearable_spritesheet.meta.image.unwrap_or_default());
        
        let wearable_image = ImageReader::open(wearable_image_path).unwrap().decode().unwrap();
        if let DynamicImage::ImageRgba8(wearable_dynamic_image) = wearable_image.clone()
        {

            for base_frame_tag in base_spritesheet.meta.frame_tags.clone().unwrap()
            {
                
                for wearable_frame_tag in wearable_spritesheet.meta.frame_tags.clone().unwrap()
                {
                    if base_frame_tag.name==wearable_frame_tag.name
                    {
                        let mut base_frame_index = base_frame_tag.from as usize;
                        let mut  wearable_frame_index = wearable_frame_tag.from as usize;

                        while base_frame_index <= base_frame_tag.to as usize && wearable_frame_index < wearable_frame_tag.to as usize
                        {

                            let base_frame = base_spritesheet.frames[base_frame_index].frame;
                            let wearable_frame = wearable_spritesheet.frames[wearable_frame_index].frame;
                        
                            let mut pixel_x = 0;
                            let mut pixel_y = 0;
                            while pixel_y<wearable_dynamic_image.dimensions().1 
                                && pixel_y <base_dynamic_image.dimensions().1 
                            {
                                while pixel_x<wearable_dynamic_image.dimensions().0 
                                    && pixel_x <base_dynamic_image.dimensions().0 
                                    {

                                        if wearable_dynamic_image.get_pixel(pixel_x, pixel_y).0[3] >0
                                        {
                                            base_dynamic_image.put_pixel(pixel_x, pixel_y, *wearable_dynamic_image.get_pixel(pixel_x, pixel_y));
                                        }
                                    
                                        pixel_x+=1; 
                                    }
                                pixel_y+=1;
                                pixel_x=0;
                            }
                        
                            base_frame_index+=1;
                            wearable_frame_index+=1;
                        }
                        break;
                    }
                    
                }
            }
        } 
    
}