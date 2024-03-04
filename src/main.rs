use ask_rs::{WaveTableOscillator, play_scale};

// A minor scale = section of A major
const A_AEOLIAN: [f32; 8] = [440.0, 493.9, 523.3, 587.3, 659.3, 698.5, 766.0, 880.0];
// C major scale
const C_IONIAN: [f32; 8] = [261.6, 293.7, 329.6, 349.2, 392.0, 440.0, 493.9, 523.3];

fn main() {
    let wave_table_length = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_length);

    // creating the sine wave table
    for n in 0..wave_table_length {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 
            / wave_table_length as f32).sin());
    }

    let mut oscillator =WaveTableOscillator::new(44100, wave_table);
    play_scale(&mut oscillator, A_AEOLIAN);
    play_scale(&mut oscillator, C_IONIAN);
}