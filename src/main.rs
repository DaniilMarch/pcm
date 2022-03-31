use std::u16;
use std::fs::File;
use byteorder::{LittleEndian, WriteBytesExt};
use pcm::prelude::*;

fn main() {
    let mut t: f64 = 0.0;
    let mut samples: Vec<f64> = vec![];
    while t < 1.5 {
        let value = AMPLITUDE * (SINE_WAVE_RATE * t).sin() + AMPLITUDE;
        samples.push(value);
        t += SAMPLING_INTERVAL;
    }

    let mut file = File::create("test.pcm").expect("Cant create file");
    for i in samples {
        let as_u16 = ((i / RANGE) * u16::MAX as f64) as u16;
        file.write_u16::<LittleEndian>(as_u16).expect("Cant write to file");
    }
}
