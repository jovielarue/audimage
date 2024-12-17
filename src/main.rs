mod dialup;
mod images;
use images::{process_rgb_vals, read_img_info};

fn main() {
    let (dimensions, pixels) = read_img_info("/home/jovie/Pictures/seeingrainbows.jpeg");
    println!("Image dimensions are {:?}x{:?}", dimensions.0, dimensions.1);

    for pixel in pixels {
        // pixel.2 is the Rgba<u8> struct from Pixel
        print!("{},{}:", pixel.0, pixel.1);
        process_rgb_vals(&pixel);
    }
}
