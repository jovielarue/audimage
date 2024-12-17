mod dialup;
mod images;
use std::collections::HashMap;

use images::{process_luma_val, read_img_info};

fn main() {
    let (dimensions, pixels) = read_img_info("/home/jovie/Pictures/seeingrainbows.jpeg");
    println!("Image dimensions are {:?}x{:?}", dimensions.0, dimensions.1);

    // pixel_map stores the luma value with a vec of all coordinates where the luma value appears
    let mut pixel_map: HashMap<u8, Vec<(u32, u32)>> = HashMap::new();

    for pixel in pixels {
        // pixel.2 is the Rgba<u8> struct from Pixel
        print!("{},{}:", pixel.0, pixel.1);
        process_luma_val(&pixel, &mut pixel_map);
    }

    println!("Pixel map is: {:?}", pixel_map);
    println!("Pixel map luma values are: {:?}", pixel_map.keys());
}
