use std::u16;
use std::fs::File;
use byteorder::{LittleEndian, WriteBytesExt};
use pcm::prelude::*;
use pcm::{write_to_wav};

//test
fn main() {
    let mut t: f64 = 0.0;
    let mut samples: Vec<f64> = vec![];
    while t < 1.5 {
        let value = AMPLITUDE * (SINE_WAVE_RATE * t).sin() + AMPLITUDE;
        samples.push(value);
        t += SAMPLING_INTERVAL;
    }

    let mut file = File::create("test.wav").expect("Cant create file");
    let mut samples_u16 = samples.into_iter().map(|x| ((x / RANGE) * u16::MAX as f64) as u16).collect();
    write_to_wav(file, samples_u16);
}
