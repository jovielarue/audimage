use image::{DynamicImage, GenericImageView, Rgba};
use std::{ops::Index, path::Path};

pub fn read_img_pixels(input_path: &str) -> Vec<(u32, u32, Rgba<u8>)> {
    let img = image::open(&Path::new(input_path)).expect("Unable to open image.");

    // can possibly use img_dimensions for transmitting data x by x or y by y
    let img = resize_img(&img);
    let img_dimensions = img.dimensions();
    println!(
        "New image dimensions are {:?}x{:?}",
        img_dimensions.0, img_dimensions.1
    );

    img.pixels().collect::<Vec<_>>()
}

pub fn resize_img(img: &DynamicImage) -> DynamicImage {
    img.resize(300, 300, image::imageops::FilterType::Nearest)
}

// maps a pixel rgb value (0-255) to a range between 400 and 3000
// this ensures all pitches are within an audible range
fn smooth_rgb_value(input: u8) -> f32 {
    let input = input as f32;
    let min_pitch: f32 = 400.0;
    let max_pitch: f32 = 3000.0;

    // get a value between 0 and 1
    let normalized = f32::max(0.0, f32::min(1.0, input / 255.0));

    // map the normalized value to the target range (between 400 and 3000)
    let output = min_pitch + normalized * (max_pitch - min_pitch);

    output
}

pub fn extract_rgb_nums_into_pitch(pixel: &Rgba<u8>) -> [f32; 3] {
    let r = *pixel.index(0);
    let g = *pixel.index(1);
    let b = *pixel.index(2);

    let pitch0 = smooth_rgb_value(r);
    let pitch1 = smooth_rgb_value(g);
    let pitch2 = smooth_rgb_value(b);
    println!("{},{},{}", pitch0, pitch1, pitch2);

    [pitch0, pitch1, pitch2]
}
