use aseprite::SpritesheetData;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer};
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum WearableType{
    Body,
    Feet,
    Lower,
    Upper,
    HeadAccesory,
    Hair
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WearableData
{
    wearable_type: WearableType,
    json_path: String,
}

pub fn make_player_spritesheet <P>(
    wearables_path: P,
    output_file: P,
) -> bool
where
    P: AsRef<Path> + Clone
{
    let mut wearables :Vec<WearableData> = Vec::new();

    let wearables_dir;

    match fs::read_dir(wearables_path.clone())
    {
        Ok(v) => wearables_dir = v,
        Err(_e) => return false
    } 
    
    for wearable_path in wearables_dir
    {
        let wearable_path_string;

        match wearable_path
        {
            Ok(v) => wearable_path_string = v.path().display().to_string(),
            Err(_e) => continue
        } 

        if wearable_path_string.to_lowercase().trim_end().ends_with("body.json")
        {   
            wearables.push(WearableData{json_path: wearable_path_string,wearable_type: WearableType::Body});
            continue;
        }

        if wearable_path_string.to_lowercase().trim_end().ends_with("feet.json")
        {
            wearables.push(WearableData{json_path: wearable_path_string,wearable_type: WearableType::Feet});
            continue;
        }
        
        if wearable_path_string.to_lowercase().trim_end().ends_with("lower.json")
        {   
            wearables.push(WearableData{json_path: wearable_path_string,wearable_type: WearableType::Lower});
            continue;
        }

        if wearable_path_string.to_lowercase().trim_end().ends_with("upper.json")
        {   
            wearables.push(WearableData{json_path: wearable_path_string,wearable_type: WearableType::Upper});
            continue;
        }

        if wearable_path_string.to_lowercase().trim_end().ends_with("headaccessory.json")
        {   
            wearables.push(WearableData{json_path: wearable_path_string,wearable_type: WearableType::HeadAccesory});
            continue;
        }

        if wearable_path_string.to_lowercase().trim_end().ends_with("hair.json")
        {   
            wearables.push(WearableData{json_path: wearable_path_string,wearable_type: WearableType::Hair});
            continue;
        }
            
    }
    
    if wearables.len() <= 0
    { 
        return false;
    }
       
    wearables.sort();

    let mut final_image: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>> = None;
    let mut final_spritesheet: Option<aseprite::SpritesheetData> = None;

  
    for wearable in wearables
    {
        
        if let Some(dynamic_image) = &mut final_image     
        {
            if let Some(spritesheet) = &final_spritesheet.clone()
            {
                add_wearable(dynamic_image,wearable.json_path.clone(),spritesheet);
            }
          
        }
        else
        {
            let file;

            match File::open(wearable.json_path.clone())
            {
                Ok(v) => file = v,
                Err(_e) => continue
            } 

            match serde_json::from_reader(file)
            {
                Ok(v) => final_spritesheet = Some(v),
                Err(_e) => continue
            } 

            let mut image_path = PathBuf::new();    
            image_path.push(wearable.json_path);
            image_path.pop();

            match final_spritesheet.clone()
            {
                Some(v) => image_path.push(v.meta.image.clone().unwrap_or_default()),
                None => continue
            
            }
            
            let image_reader;
            match ImageReader::open(image_path)
            {
                Ok(v) => image_reader = v,
                Err(_e) => return false
            }

            if let  DynamicImage::ImageRgba8(dynamic_image) =  image_reader.decode().unwrap()
           {
                final_image = Some(dynamic_image);
           }
        }

    }       
        
         
    let mut output_image_path = PathBuf::new();    
    output_image_path.push(output_file.clone());

    let mut output_image_name = output_image_path.file_stem().unwrap_or_default().to_str().unwrap_or_default().to_owned();
    output_image_name =  output_image_name + ".png";

    output_image_path.pop();
    output_image_path.push(output_image_name.clone());

    let writer;

    match File::create(output_file)
    {
        Ok(v) => writer = v,
        Err(_e) => return false
    } 

    let new_image = Some(output_image_name);

    match final_spritesheet.as_mut()
    {
        Some(v) => v.meta.image = new_image,
        None => return false
    
    }

    match serde_json::to_writer(&writer, &final_spritesheet)
    {
        Ok(_v) => println!("saved player json"),
        Err(_e) => return false
    }
    
    if let Some(dynamic_image) = final_image
    {
       
        match  dynamic_image.save(output_image_path)
        {
            Ok(_v) => println!("saved player image"),
            Err(_e) => return false
        }
    }

    return true;

}


fn add_wearable <P> (base_dynamic_image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,  wearable_path:P, base_spritesheet: &SpritesheetData)
where
    P: AsRef<Path> + Clone
{
    let wearable_file: File;

    match  File::open(wearable_path.clone())
    {
        Ok(v) => wearable_file = v,
        Err(_e) => return
    }


    let wearable_spritesheet: aseprite::SpritesheetData;

    match  serde_json::from_reader(wearable_file)
    {
        Ok(v) => wearable_spritesheet = v,
        Err(_e) => return
    }

    let mut wearable_image_path = PathBuf::new();    
    wearable_image_path.push(wearable_path);
    wearable_image_path.pop();
    wearable_image_path.push(&wearable_spritesheet.meta.image.unwrap_or_default());
    
    let wearable_image;

    match  ImageReader::open(wearable_image_path)
    {
        Ok(v) => wearable_image = v.decode().unwrap_or_default(),
        Err(_e) => return
    };

    if let DynamicImage::ImageRgba8(wearable_dynamic_image) = wearable_image.clone()
    {

        for base_frame_tag in base_spritesheet.meta.frame_tags.clone().unwrap_or_default()
        {
            
            for wearable_frame_tag in wearable_spritesheet.meta.frame_tags.clone().unwrap_or_default()
            {
                if base_frame_tag.name==wearable_frame_tag.name
                {
                    let mut base_frame_index = base_frame_tag.from as usize;
                    let mut  wearable_frame_index = wearable_frame_tag.from as usize;

                    while base_frame_index <= base_frame_tag.to as usize && wearable_frame_index <= wearable_frame_tag.to as usize
                    {

                        let base_frame = base_spritesheet.frames[base_frame_index].frame;
                        let wearable_frame = wearable_spritesheet.frames[wearable_frame_index].frame;
                    
                        let mut pixel_x = base_frame.x;
                        let mut pixel_y = base_frame.y;
                        while  pixel_y < wearable_dynamic_image.dimensions().1  
                            && pixel_y <base_dynamic_image.dimensions().1 
                            && pixel_y < base_frame.h+base_frame.y
                            && pixel_y < wearable_frame.h+wearable_frame.y
                        {
                            while pixel_x<wearable_dynamic_image.dimensions().0 
                                && pixel_x <base_dynamic_image.dimensions().0 
                                && pixel_x < base_frame.w+base_frame.x
                                && pixel_x < wearable_frame.w+wearable_frame.x
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