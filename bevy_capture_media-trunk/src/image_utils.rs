use bevy::render::texture::TextureFormatPixelInfo;
use image::RgbaImage;
use wgpu::TextureFormat;

pub fn frame_data_to_rgba_image(
	width: u32,
	height: u32,
	buffer: Vec<u8>,
	format: TextureFormat,
) -> RgbaImage {
	let pixels = buffer.chunks(format.pixel_size()).collect::<Vec<&[u8]>>();
	RgbaImage::from_fn(width, height, |x, y| {
		let index = ((y * width) + x) as usize;
		let pixel = pixels[index];

		match format {
			TextureFormat::Rgba8UnormSrgb
			| TextureFormat::Rgba8Uint
			| TextureFormat::Rgba8Sint
			| TextureFormat::Rgba8Snorm
			| TextureFormat::Rgba8Unorm => image::Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]),
			TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
				image::Rgba([pixel[2], pixel[1], pixel[0], pixel[3]])
			}
			_ => {
				panic!("Unhandled texture format {:?}", format);
			}
		}
	})
}
