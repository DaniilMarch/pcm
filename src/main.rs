use std::f64::consts::PI;
use std::u16;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use byteorder::{LittleEndian, WriteBytesExt};

const SAMPLING_RATE: u32 = 44_100; //Hz
const SAMPLING_INTERVAL: f64 = 1.0 / SAMPLING_RATE as f64;
const A440_RATE: u32 = 440; //Hz
const SAMPLE_SIZE: u32 = 16; //bits
const RANGE: f64 = 2.0;
const AMPLITUDE: f64 = RANGE / 2.0;
const WAVE_DURATION: f64 = 1.0 / A440_RATE as f64;
const SINE_WAVE_RATE: f64 = 2.0 * PI / WAVE_DURATION;

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
