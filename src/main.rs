mod images;
mod pitches;
use images::{extract_rgb_nums_into_pitch, read_img_pixels};
use pitches::play_rgb_tone;
use rodio::{OutputStream, Sink};
use std::time::Duration;

fn main() {
    // stream and sink are for outputting audio
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Unable to get default output stream.");
    let sink = Sink::try_new(&stream_handle).expect("Unable to get new sink.");

    let pixels = read_img_pixels("/home/jovie/Pictures/seeingrainbows.jpeg");
    for pixel in pixels {
        // pixel.2 is the Rgba<u8> struct from Pixel
        let pitches = extract_rgb_nums_into_pitch(&pixel.2);

        let mut i = 1;
        for pitch in pitches {
            // every 3rd note (after r,g,b have been played) play a reference note
            if i % 3 == 0 {
                play_rgb_tone(350.0, Duration::from_millis(300), 0.2, &sink);
            }

            play_rgb_tone(pitch, Duration::from_millis(80), 0.1, &sink);
            i += 1;
        }
    }
}
