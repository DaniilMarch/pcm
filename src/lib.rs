use std::fs::File;
use byteorder::{LittleEndian, WriteBytesExt, BigEndian};
use std::io::Write;

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
use prelude::*;

/* 
    WAV structure
    Endian      File offset (bytes)     Field Name      Field size (bytes)      Content
    big         0                       ChunkId         4                       "RIFF" in ASCII (0x52494646)
    little      4                       ChunkSize       4                       36 + SubChunk2Size
    big         8                       Format          4                       "WAVE" in ASCII (0x57415645)
    big         12                      SubChunk1ID     4                       "fmt" in in ASCII (0x666d7420)
    little      16                      SubChunk1Size   4                       16 for PCM
    little      20                      AudioFormat     2                       1 for PCM
    little      22                      NumOfChannels   2                       1 for mono, 2 for stereo
    little      24                      SampleRate      4                       8000, 44100, etc
    little      28                      ByteRate        4                       == Sample Rate * NumChannels * BitsPerSample / 8
    little      32                      BlockAlign      2                       == NumChannels * BitsPerSample / 8
    little      34                      BitsPerSample   2                       8 = 8 bits, 16 = 16 bits, etc
    big         36                      SubChunk2ID     4                       "data" in ASCII (0x64617461)
    little      40                      SubChunk2Size   4                       == NumSamples * NumChannels * BitsPerSample / 8
    little      44                      Data            *                       Actual sound data
*/
pub fn write_to_wav(mut file: File, data: Vec<u16>) {
    file.write_all(b"RIFF").expect("Failed to write RIFF header");
    file.write_u32::<LittleEndian>(36 + data.len() as u32 * 1 * 2).expect("Failed to write"); //36 + NumSamples (data.len()) * NumChannels (1, mono for now) * BitsPerSample / 8 (16 / 8 = 2)
    file.write_all(b"WAVE").expect("Failed to write WAVE header");
    file.write_all(b"fmt ").expect("Failed to write fmt");
    file.write_u32::<LittleEndian>(16).expect("Failed to write SubChunk1Size");
    file.write_u16::<LittleEndian>(1).expect("Failed to write audio format");
    file.write_u16::<LittleEndian>(1).expect("Failed to write num channels");
    file.write_u32::<LittleEndian>(SAMPLING_RATE).expect("Failed to write sample rate");
    file.write_u32::<LittleEndian>(SAMPLING_RATE * 1 * 2).expect("Failed to write byte rate");
    file.write_u16::<LittleEndian>(1 * 2).expect("Failed to write block align");
    file.write_u16::<LittleEndian>(16).expect("Failed to write bits per sample");
    file.write_all(b"data").expect("Failed to write SubChunk2Id");
    file.write_u32::<LittleEndian>(data.len() as u32 * 1 * 2).expect("Failed to write SubChunk2Size");
    for sample in data {
        file.write_u16::<LittleEndian>(sample);
    }
}