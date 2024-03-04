use std::time::Duration;
use ask_rs::WaveTableOscillator;
use rodio::{OutputStream, Source};

fn main() {
    let wave_table_length = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_length);

    // creating the sine wave table
    for n in 0..wave_table_length {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 
            / wave_table_length as f32).sin());
    }

    let mut oscillator = WaveTableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0); // TODO: MAKE VARIABLE FREQUENCY

    
    // SHOULDN'T BE HERE BUT IT NEEDS TO
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());
    std::thread::sleep(Duration::from_secs(5))
}
