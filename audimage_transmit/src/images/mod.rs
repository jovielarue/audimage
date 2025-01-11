use image::{DynamicImage, GenericImageView, Luma};
use std::{collections::BTreeMap, path::Path};

pub fn read_img_info(input_path: &str) -> ((u32, u32), Vec<(u32, u32, Luma<u8>)>) {
    let img = image::open(&Path::new(input_path)).expect("Unable to open image.");

    let resized_img = resize_img(&img);
    // convert img to Luma (greyscale)
    let luma_img = resized_img.to_luma8();
    let (width, height) = resized_img.dimensions();

    // Get a vec with (x,y) coords along with its luma value
    let pixels: Vec<(u32, u32, Luma<u8>)> = luma_img
        .enumerate_pixels()
        .map(|(x, y, pixel)| (x, y, Luma::from(pixel.clone())))
        .collect();

    // return (img dimensions) and pixels vec
    ((width, height), pixels)
}

pub fn resize_img(img: &DynamicImage) -> DynamicImage {
    img.resize(50, 50, image::imageops::FilterType::Gaussian)
}

// insert a pixel into the BTree with luma_val:Vec<coordinate> pairs
pub fn process_luma_val(
    pixel: &(u32, u32, Luma<u8>),
    pixel_map: &mut BTreeMap<u8, Vec<(u32, u32)>>,
) {
    let luma_val = pixel.2[0];
    let luma_coords = (pixel.0, pixel.1);

    pixel_map
        .entry(luma_val)
        .and_modify(|coords| coords.push(luma_coords))
        .or_insert(vec![luma_coords]);
}
