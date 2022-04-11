#![warn(clippy::all, clippy::pedantic)]
use pcm::{
    write_to_wav,
    write_to_mp3,
    sample_note_sequence,
    AudioParams, 
    NoteFrequencyParams, 
    NoteMusicalNotation, 
    NoteLitera
};
use std::fs::File;
use std::u16;
use regex::Regex;

fn main() {
    let string_sequence = "4A1,2B#1,1C1";
    let regex = Regex::new(r"(?P<duration>^\d+)(?P<litera>[ABCDEFG]+)(?P<alter>\#?)(?P<octave>\d?$)").unwrap();
    
    let note_sequence = string_sequence.split(",").map(|sound| {
        NoteFrequencyParams::from_tab_string(&regex, sound)
    }).collect::<Vec<NoteFrequencyParams>>();

    let params = AudioParams::new(44_100, 16, 1, 2.0);
    let samples = sample_note_sequence(&params, note_sequence);

    let file = File::create("test.wav").expect("Cant create file");
    let samples_u16 = samples
        .into_iter()
        .map(|x| ((x / params.get_range()) * u16::MAX as f64) as u16)
        .collect::<Vec<u16>>();
    if let Err(why) = write_to_wav(file, params, samples_u16) {
        println!("Failed to write to wav: {}", why);
    }
}
