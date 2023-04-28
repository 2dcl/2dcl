use aseprite::SpritesheetData;
use dcl_common::Result;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer};
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use super::error::SpriteMakerError;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum WearableType {
    Background,
    Body,
    Feet,
    Lower,
    Upper,
    HeadAccesory,
    Hair,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WearableData {
    wearable_type: WearableType,
    json_path: PathBuf,
}

fn get_wearables_data<P>(wearables_path: P) -> Result<Vec<WearableData>>
where
    P: AsRef<Path>,
{
    let mut wearables: Vec<WearableData> = Vec::new();
    let wearables_dir = match fs::read_dir(wearables_path) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    for wearable_path in wearables_dir {
        let wearable_path = match wearable_path {
            Ok(v) => v.path(),
            Err(_e) => continue,
        };

        let file_extension = match wearable_path.extension() {
            Some(v) => v,
            None => continue,
        };

        if file_extension != "json" {
            continue;
        }

        let file_name = match wearable_path.file_stem() {
            Some(v) => v,
            None => continue,
        };

        let file_name = match file_name.to_str() {
            Some(v) => v.to_lowercase(),
            None => continue,
        };

        if file_name.ends_with("body") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::Body,
            });
            continue;
        }

        if file_name.ends_with("feet") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::Feet,
            });
            continue;
        }

        if file_name.ends_with("lower") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::Lower,
            });
            continue;
        }

        if file_name.ends_with("upper") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::Upper,
            });
            continue;
        }

        if file_name.ends_with("headaccessory") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::HeadAccesory,
            });
            continue;
        }

        if file_name.ends_with("hair") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::Hair,
            });
            continue;
        }

        if file_name.ends_with("background") || file_name.ends_with("bg") {
            wearables.push(WearableData {
                json_path: wearable_path,
                wearable_type: WearableType::Background,
            });
            continue;
        }
    }
    wearables.sort();
    Ok(wearables)
}
pub fn make_player_spritesheet<P>(wearables_path: P, output_file: P) -> Result<()>
where
    P: AsRef<Path> + Clone + Debug,
{
    let wearables = match get_wearables_data(wearables_path) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    if wearables.is_empty() {
        return Err(Box::new(SpriteMakerError::NoWearables));
    }

    let mut body_wearable = None;

    for wearable in &wearables {
        if wearable.wearable_type == WearableType::Body {
            body_wearable = Some(wearable);
        }
    }

    if body_wearable.is_none() {
        return Err(Box::new(SpriteMakerError::NoBody));
    }

    let body_wearable = body_wearable.unwrap();

    let file = match File::open(&body_wearable.json_path) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let mut final_spritesheet: aseprite::SpritesheetData = match serde_json::from_reader(file) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let mut image_path = PathBuf::from(&body_wearable.json_path);
    image_path.pop();
    image_path.push(final_spritesheet.meta.image.clone().unwrap_or_default());

    let image_reader = match ImageReader::open(&image_path) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let image_reader = match image_reader.decode() {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let mut final_image = match image_reader {
        DynamicImage::ImageRgba8(v) => v,
        _ => return Err(Box::new(SpriteMakerError::InvalidImageFormat(image_path))),
    };

    for wearable in wearables {
        add_wearable(
            &mut final_image,
            wearable.json_path.clone(),
            &final_spritesheet,
        );
    }

    let mut output_image_path = PathBuf::new();
    output_image_path.push(output_file.clone());

    let mut output_image_name = output_image_path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_owned();
    output_image_name += ".png";

    output_image_path.pop();
    output_image_path.push(output_image_name.clone());

    let writer = match File::create(output_file) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    final_spritesheet.meta.image = Some(output_image_name);

    if let Err(e) = serde_json::to_writer(&writer, &final_spritesheet) {
        return Err(Box::new(e));
    }

    if let Err(e) = final_image.save(output_image_path) {
        return Err(Box::new(e));
    }

    Ok(())
}

fn add_wearable<P>(
    base_dynamic_image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    wearable_path: P,
    base_spritesheet: &SpritesheetData,
) where
    P: AsRef<Path> + Clone,
{
    let wearable_file = match File::open(wearable_path.clone()) {
        Ok(v) => v,
        Err(_e) => return,
    };

    let wearable_spritesheet: aseprite::SpritesheetData =
        match serde_json::from_reader(wearable_file) {
            Ok(v) => v,
            Err(_e) => return,
        };

    let mut wearable_image_path = PathBuf::new();
    wearable_image_path.push(wearable_path);
    wearable_image_path.pop();
    wearable_image_path.push(&wearable_spritesheet.clone().meta.image.unwrap_or_default());

    let wearable_image = match ImageReader::open(wearable_image_path) {
        Ok(v) => v.decode().unwrap_or_default(),
        Err(_e) => return,
    };

    if let DynamicImage::ImageRgba8(wearable_dynamic_image) = wearable_image {
        for base_frame_tag in base_spritesheet.meta.frame_tags.clone().unwrap_or_default() {
            for wearable_frame_tag in wearable_spritesheet
                .clone()
                .meta
                .frame_tags
                .clone()
                .unwrap_or_default()
            {
                if base_frame_tag.name == wearable_frame_tag.name {
                    let mut base_frame_index = base_frame_tag.from as usize;
                    let mut wearable_frame_index = wearable_frame_tag.from as usize;

                    while base_frame_index <= base_frame_tag.to as usize
                        && wearable_frame_index <= wearable_frame_tag.to as usize
                    {
                        let base_frame = base_spritesheet.frames[base_frame_index].frame;
                        let wearable_frame =
                            wearable_spritesheet.frames[wearable_frame_index].frame;
                        let max_x = base_frame.w.min(wearable_frame.w);
                        let max_y = base_frame.h.min(wearable_frame.h);

                        for x_index in 0..max_x {
                            for y_index in 0..max_y {
                                let wearable_pixel = wearable_dynamic_image.get_pixel(
                                    x_index + wearable_frame.x,
                                    y_index + wearable_frame.y,
                                );

                                if wearable_pixel.0[3] > 0 {
                                    base_dynamic_image.put_pixel(
                                        x_index + base_frame.x,
                                        y_index + base_frame.y,
                                        *wearable_pixel,
                                    );
                                }
                            }
                        }
                        base_frame_index += 1;
                        wearable_frame_index += 1;
                    }
                    break;
                }
            }
        }
    }
}
