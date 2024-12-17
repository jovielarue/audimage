use image::{DynamicImage, GenericImageView, Luma};
use std::path::Path;

pub fn read_img_info(input_path: &str) -> ((u32, u32), Vec<(u32, u32, Luma<u8>)>) {
    let img = image::open(&Path::new(input_path)).expect("Unable to open image.");

    // can possibly use img_dimensions for transmitting data x by x or y by y
    let resized_img = resize_img(&img);
    let luma_img = resized_img.to_luma8();
    let (width, height) = img.dimensions();

    let pixels: Vec<(u32, u32, Luma<u8>)> = luma_img
        .enumerate_pixels()
        .map(|(x, y, pixel)| (x, y, Luma::from(pixel.clone())))
        .collect();

    ((width, height), pixels)
}

pub fn resize_img(img: &DynamicImage) -> DynamicImage {
    img.resize(100, 100, image::imageops::FilterType::Nearest)
}

pub fn process_rgb_vals(pixel: &(u32, u32, Luma<u8>)) {
    let luma_val = pixel.2[0];

    println!("{:?}", luma_val);
}
