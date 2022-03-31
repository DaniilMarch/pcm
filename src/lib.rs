pub mod prelude {
    use std::f64::consts::PI;
    pub const SAMPLING_RATE: u32 = 44_100; //Hz
    pub const SAMPLING_INTERVAL: f64 = 1.0 / SAMPLING_RATE as f64;
    pub const A440_RATE: u32 = 440; //Hz
    pub const SAMPLE_SIZE: u32 = 16; //bits
    pub const RANGE: f64 = 2.0;
    pub const AMPLITUDE: f64 = RANGE / 2.0;
    pub const WAVE_DURATION: f64 = 1.0 / A440_RATE as f64;
    pub const SINE_WAVE_RATE: f64 = 2.0 * PI / WAVE_DURATION;
}