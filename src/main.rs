#![warn(clippy::all, clippy::pedantic)]
use pcm::{write_to_wav, AudioParams, Note};
use std::fs::File;
use std::u16;

fn main() {
    let params = AudioParams::new(44_100, 16, 1, 2.0);
    let a440 = Note::new(440);

    let mut t: f64 = 0.0;
    let mut samples: Vec<f64> = vec![];
    let amplitude = params.get_amplitude();
    while t < 1.5 {
        let value = amplitude * (a440.get_frequency() * t).sin() + amplitude;
        samples.push(value);
        t += params.get_sampling_interval();
    }

    let file = File::create("test.wav").expect("Cant create file");
    let samples_u16 = samples
        .into_iter()
        .map(|x| ((x / params.get_range()) * u16::MAX as f64) as u16)
        .collect();
    if let Err(why) = write_to_wav(file, params, samples_u16) {
        println!("Failed to write to wav: {}", why);
    }
}
