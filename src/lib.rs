use byteorder::{LittleEndian, WriteBytesExt};
use std::f64::consts::PI;
use std::io::{BufWriter, Error, Write};
use regex::Regex;

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
#[derive(Clone, Copy, Debug)]
pub enum NoteLitera {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<NoteLitera> for i32 {
    fn from(note: NoteLitera) -> i32 {
        match note {
            NoteLitera::C => -9,
            NoteLitera::D => -7,
            NoteLitera::E => -5,
            NoteLitera::F => -4,
            NoteLitera::G => -2,
            NoteLitera::A => 0,
            NoteLitera::B => 2,
        }
    }
}


#[allow(dead_code)]
#[derive(Debug)]
pub enum NoteAlter {
    Sharp,
    Flat,
}

#[derive(Debug)]
pub struct NoteMusicalNotation {
    litera: NoteLitera,
    alter: Option<NoteAlter>,
    octave: u32,
}

impl NoteMusicalNotation {
    pub fn new(litera: NoteLitera, alter: Option<NoteAlter>, octave: u32) -> NoteMusicalNotation {
        NoteMusicalNotation {
            litera,
            alter,
            octave,
        }
    }
}

#[derive(Debug)]
pub struct NoteFrequencyParams {
    rate: f64,
    length: f64,
    frequency: f64,
    duration: f64,
}

const ORIGIN_FREQUENCY: f64 = 440.0;
const ORIGIN_NOTE_LITERA: NoteLitera = NoteLitera::A;
const ORIGIN_NOTE_OCTAVE: i32 = 4; //From fact that 'first' octave is actually fourth on piano keyboard counting from 0
const ORIGIN_NOTE_VALUE: u8 = 0;

impl NoteFrequencyParams {
    pub fn new(rate: f64, duration: f64) -> NoteFrequencyParams {
        let length = 1.0 / rate;
        let frequency = 2.0 * PI / length;

        NoteFrequencyParams {
            rate,
            length,
            frequency,
            duration,
        }
    }

    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }

    pub fn get_duration(&self) -> f64 {
        self.duration
    }

    pub fn calculate_distance_from_origin(note_musical_notation: NoteMusicalNotation) -> i32 {
        let litera_value = i32::from(note_musical_notation.litera);
        let octave_distance = ORIGIN_NOTE_OCTAVE - note_musical_notation.octave as i32;
        litera_value - 12 * octave_distance + if let Some(alter) = note_musical_notation.alter {
            match alter {
                NoteAlter::Flat => -1,
                NoteAlter::Sharp => 1,
            }
        } else { 0 }
    }

    pub fn new_from_musical_notation(note_musical_notation: NoteMusicalNotation, duration: f64) -> NoteFrequencyParams {
        let distance_from_origin = NoteFrequencyParams::calculate_distance_from_origin(note_musical_notation);
        let rate = ORIGIN_FREQUENCY * 2.0_f64.powf(distance_from_origin as f64 / 12.0);
        NoteFrequencyParams::new(rate, duration)
    }

    pub fn from_tab_string(regex: &Regex, s: &str) -> NoteFrequencyParams {
        let caps = regex.captures(s).unwrap();
        let litera = caps.name("litera").unwrap().as_str();
        let alter = caps.name("alter").unwrap().as_str();
        let octave = caps.name("octave").unwrap().as_str().parse::<u32>().unwrap();
        let duration = caps.name("duration").unwrap().as_str().parse::<f64>().unwrap();

        let litera = match litera {
            "A" => NoteLitera::A,
            "B" => NoteLitera::B,
            "C" => NoteLitera::C,
            "D" => NoteLitera::D,
            "E" => NoteLitera::E,
            "F" => NoteLitera::F,
            "G" => NoteLitera::G,
            _ => panic!("Wrong note")
        };
        let alter = match alter {
            "#" => Some(NoteAlter::Sharp),
            "b" => Some(NoteAlter::Flat),
            _ => None,
        };


        let musical_notation = NoteMusicalNotation::new(litera, alter, octave);
        println!("{:?}", musical_notation);
        println!("{}", duration);
        NoteFrequencyParams::new_from_musical_notation(musical_notation, duration)
    }
}

pub fn sample_note_sequence(audio_params: &AudioParams, notes: Vec<NoteFrequencyParams>) -> Vec<f64> {
    let mut result = vec![];
    for note in notes {
        let mut t: f64 = 0.0;
        let amplitude = audio_params.get_amplitude();
        while t < note.get_duration() {
            let value = amplitude * (note.get_frequency() * t).sin() + amplitude;
            result.push(value);
            t += audio_params.get_sampling_interval();
        }
    }
    result
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

pub fn write_to_mp3<T: Write>(out: T) -> Result<(), Error>{
    let mut writer = BufWriter::new(out);

    let mut header: u32 = 0b1;
    for i in 0..11 { //Mp3 sync word which is 1111_1111_1111
        header <<= 1;
        header |= 0b1;
    }
    header <<= 1;
    header |= 0b1; //Set 13th bit 1 for MPEG version 1

    header <<= 2;
    header |= 0b1; //Set 14 and 15 to 01 for Layer III

    header <<= 1;
    header |= 0b1; //Set 16 bit for no error protection

    header <<= 4;
    header |= 0b1010; //Set 1010 for 160 bit rate

    header <<= 4; // 00 for 44100 Hz sampling rate, 0 for frame padding, 0 for private bit
    
    header <<= 2;
    header |= 0b1; //01 for Joint stereo

    header <<= 2; // 00 for no mode extensions

    header <<= 4; // 0 for not copy-righted, 0 for copy, 00 for emphasis

    println!("{:b}", header);
    

    writer.flush()?;
    Ok(())
}