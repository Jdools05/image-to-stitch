use image::{DynamicImage, Rgba, imageops::{resize, FilterType}};
use rayon::prelude::*;
use imageproc::filter::gaussian_blur_f32;

use crate::{color_mapping::ColorMapping, color_threads::ColorThread};
mod color_mapping;
mod color_threads;


mod utils {
    macro_rules! dehex_color {
        ($hex:expr) => {
            if $hex.len() == 7 {
                Rgba([
                    u8::from_str_radix(&$hex[1..3], 16).unwrap(),
                    u8::from_str_radix(&$hex[3..5], 16).unwrap(),
                    u8::from_str_radix(&$hex[5..7], 16).unwrap(),
                    255,
                ])
            } else {
                Rgba([
                    u8::from_str_radix(&$hex[1..3], 16).unwrap(),
                    u8::from_str_radix(&$hex[3..5], 16).unwrap(),
                    u8::from_str_radix(&$hex[5..7], 16).unwrap(),
                    u8::from_str_radix(&$hex[7..9], 16).unwrap(),
                ])
            }
        };
    }

    pub(crate) use dehex_color;
}

fn main() {
    // the path to the image file
    let image_path: &str = "src/media/test.jpg";

    // load the image
    let image: DynamicImage = image::open(image_path).unwrap();

    // convert the DynamicImage to an ImageBuffer
    let image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image.to_rgba8();

    // blur the image
    let blurred_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = resize(&gaussian_blur_f32(&image_buffer, 1.0), 100, 100, FilterType::Lanczos3);

    println!("Image dimensions: {:?}x{:?}", image.width(), image.height());

    // create our color mapping
    let color_mapping: ColorMapping = ColorMapping::from_threads(ColorThread::load_from_file("src/thread_colors.csv"));

    // process the image in parallel while keeping track of the pixel indices
    let processed_pixels: Vec<(usize, Rgba<u8>)> = blurred_image
        .pixels()
        .enumerate()
        .par_bridge()
        .map(|(index, pixel)| {
            // get the color of the pixel
            let color: Rgba<u8> = ColorMapping::get_color(&color_mapping, Rgba(pixel.0));
            (index, color)
        })
        .collect();

    // sort the processed pixels based on their original indices
    let mut sorted_pixels: Vec<Rgba<u8>> = vec![Rgba([0, 0, 0, 0]); processed_pixels.len()];
    for (index, pixel) in processed_pixels {
        sorted_pixels[index] = pixel;
    }

    // reform the Vec<Rgba<u8>> into a [u8]
    let processed_pixels: Vec<u8> = sorted_pixels
        .into_iter()
        .flat_map(|pixel| vec![pixel[0], pixel[1], pixel[2], pixel[3]])
        .collect();

    // save the processed pixels directly to a file
    image::save_buffer_with_format(
        "src/media/test.png",
        &processed_pixels,
        blurred_image.width(),
        blurred_image.height(),
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();

    println!("Done!");

    // make the color threads
    let color_threads: Vec<color_threads::ColorThread> = color_threads::ColorThread::load_from_file("src/thread_colors.csv");

    for color_thread in color_threads {
        println!("{}: {} and hex: {}", color_thread.get_floss(), color_thread.get_name(), color_thread.get_hex());
    }

}

fn generate_color_mapping(color_depth: u16) -> ColorMapping {
    let mut color_map: ColorMapping = ColorMapping::new(Vec::new());

    for r in 0..=color_depth {
        for g in 0..=color_depth {
            for b in 0..=color_depth {
                let hex: String = format!("#{:02x}{:02x}{:02x}", r * 255 / color_depth, g * 255 / color_depth, b * 255 / color_depth);
                // add the color to the mapping
                color_map.push(utils::dehex_color!(hex));          
            }
        }
    }

    println!("Color map size: {}", color_map.len());

    color_map
}