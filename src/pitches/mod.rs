use rodio::source::{SineWave, Source};
use rodio::Sink;
use std::{thread, time::Duration};

pub fn play_rgb_tone(pitch: f32, duration: Duration, amplification: f32, sink: &Sink) {
    let source = SineWave::new(pitch)
        .take_duration(duration)
        .amplify(amplification);

    // appending source to sink queues the source for playback
    sink.append(source);
    thread::sleep(duration);
}
