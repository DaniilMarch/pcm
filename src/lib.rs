use byteorder::{LittleEndian, WriteBytesExt};
use std::f64::consts::PI;
use std::io::{BufWriter, Error, Write};

pub struct AudioParams {
    sample_rate: u32,
    bits_per_sample: u32,
    number_of_channels: u32,
    sampling_interval: f64,
    range: f64,
    amplitude: f64,
}

impl AudioParams {
    pub fn new(
        sample_rate: u32,
        bits_per_sample: u32,
        number_of_channels: u32,
        range: f64,
    ) -> AudioParams {
        AudioParams {
            sample_rate,
            bits_per_sample,
            number_of_channels,
            range,
            sampling_interval: 1.0 / sample_rate as f64,
            amplitude: range / 2.0,
        }
    }

    pub fn get_amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn get_sampling_interval(&self) -> f64 {
        self.sampling_interval
    }

    pub fn get_range(&self) -> f64 {
        self.range
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_number_of_channels(&self) -> u32 {
        self.number_of_channels
    }

    pub fn get_bits_per_sample(&self) -> u32 {
        self.bits_per_sample
    }
}

#[allow(dead_code)]
enum Notes {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[allow(dead_code)]
enum NoteAlter {
    Sharp,
    Flat,
}

#[allow(dead_code)]
pub struct Note {
    rate: u32,
    duration: f64,
    frequency: f64,
}

#[allow(dead_code)]
const ORIGIN_FREQUENCY: u32 = 440;
impl Note {
    pub fn new(rate: u32) -> Note {
        let duration = 1.0 / rate as f64;
        let frequency = 2.0 * PI / duration;

        Note {
            rate,
            duration,
            frequency,
        }
    }

    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }

    pub fn calculate_distance() -> u32 {
        1
    }
}

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

const SUBCHUNK1_SIZE: u32 = 16;
pub fn write_to_wav<T: Write>(out: T, params: AudioParams, data: Vec<u16>) -> Result<(), Error> {
    let subchunk2_size =
        data.len() as u32 * params.get_number_of_channels() * params.get_bits_per_sample();
    let num_channels_bps_product =
        params.get_number_of_channels() * params.get_bits_per_sample() / 8;

    let mut writer = BufWriter::new(out);
    writer.write_all(b"RIFF")?;
    writer.write_u32::<LittleEndian>(36 + subchunk2_size)?; //36 + NumSamples (data.len()) * NumChannels (1, mono for now) * BitsPerSample / 8 (16 / 8 = 2)
    writer.write_all(b"WAVE")?;
    writer.write_all(b"fmt ")?;
    writer.write_u32::<LittleEndian>(SUBCHUNK1_SIZE)?;
    writer.write_u16::<LittleEndian>(1)?;
    writer.write_u16::<LittleEndian>(params.get_number_of_channels() as u16)?;
    writer.write_u32::<LittleEndian>(params.get_sample_rate())?;
    writer.write_u32::<LittleEndian>(params.get_sample_rate() * num_channels_bps_product)?;
    writer.write_u16::<LittleEndian>(num_channels_bps_product as u16)?;
    writer.write_u16::<LittleEndian>(params.get_bits_per_sample() as u16)?;
    writer.write_all(b"data")?;
    writer.write_u32::<LittleEndian>(data.len() as u32 * num_channels_bps_product)?;
    for sample in data {
        writer.write_u16::<LittleEndian>(sample)?;
    }
    writer.flush()?;
    Ok(())
}
