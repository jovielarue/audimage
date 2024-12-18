mod images;
use std::collections::BTreeMap;

use images::{process_luma_val, read_img_info};

fn main() {
    let (dimensions, pixels) = read_img_info("/home/jovie/Pictures/seeingrainbows.jpeg");
    println!("Image dimensions are {:?}x{:?}", dimensions.0, dimensions.1);

    // pixel_map stores the luma value with a vec of all coordinates where the luma value appears
    let mut pixel_map: BTreeMap<u8, Vec<(u32, u32)>> = BTreeMap::new();

    for pixel in pixels {
        // pixel.2 is the Rgba<u8> struct from Pixel
        process_luma_val(&pixel, &mut pixel_map);
    }

    let mut curr_luma: u8 = 255;
    for pixel in pixel_map {
        for coord in pixel.1 {
            if curr_luma != pixel.0 {
                println!("Coordinates for luma val: {}", pixel.0)
            }
            println!("{},{}", coord.0, coord.1);
            curr_luma = pixel.0;
        }
    }
}
