#![warn(clippy::all, clippy::pedantic)]
use pcm::{
    write_to_wav,
    sample_note_sequence,
    AudioParams, 
    NoteFrequencyParams, 
    NoteMusicalNotation, 
    NoteLitera
};
use std::fs::File;
use std::u16;

fn main() {
    let params = AudioParams::new(44_100, 16, 1, 2.0);
    let samples = sample_note_sequence(&params, vec![
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::C, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::D, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::E, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::F, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::G, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::A, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::B, None, 4)), 1.0),
        (NoteFrequencyParams::new_from_musical_notation(NoteMusicalNotation::new(NoteLitera::C, None, 5)), 1.0),
    ]);

    let file = File::create("test.wav").expect("Cant create file");
    let samples_u16 = samples
        .into_iter()
        .map(|x| ((x / params.get_range()) * u16::MAX as f64) as u16)
        .collect();
    if let Err(why) = write_to_wav(file, params, samples_u16) {
        println!("Failed to write to wav: {}", why);
    }
}
