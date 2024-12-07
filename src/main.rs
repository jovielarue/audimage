use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Pixels, Rgba};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::cmp::max;
use std::ops::Index;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Unable to get default output stream.");
    let sink = Sink::try_new(&stream_handle).expect("Unable to get new sink.");

    let pixels = read_img_pixels("/home/jovie/Pictures/seeingrainbows.jpeg");
    for pixel in pixels {
        let pitches = extract_rgb_nums_into_pitch(&pixel.2);

        for pitch in pitches {
            play_rgb_tone(pitch as f32, &sink);
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn read_img_pixels(input_path: &str) -> Vec<(u32, u32, Rgba<u8>)> {
    let img = image::open(&Path::new(input_path)).expect("Unable to open image.");

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;
    println!("{}, {}", img_width, img_height);

    img.pixels().collect::<Vec<_>>()
}

fn play_rgb_tone(pitch: f32, sink: &Sink) {
    let source = SineWave::new(pitch)
        .take_duration(Duration::from_millis(100))
        .amplify(0.1);

    sink.append(source);
    sink.sleep_until_end();
}

fn extract_rgb_nums_into_pitch(pixel: &Rgba<u8>) -> [f32; 3] {
    let r = *pixel.index(0) as u32;
    let g = *pixel.index(1) as u32;
    let b = *pixel.index(2) as u32;

    let pitch0 = max(r * 10 + 100, 1000) as f32;
    let pitch1 = max(g * 10 + 100, 1000) as f32;
    let pitch2 = max(b * 10 + 100, 1000) as f32;
    println!("{},{},{}", pitch0, pitch1, pitch2);

    [pitch0, pitch1, pitch2]
}
