// credits for the wave table synthesis tutorial to WolfSound.
// WolfSound youtube channel: https://www.youtube.com/@WolfSoundAudio

use rodio::source::Source;

pub struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscillator {
        WaveTableOscillator { sample_rate, wave_table, 
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, f: f32) {
        self.index_increment = f * self.wave_table.len() as f32 /
            self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;

        sample
    }

    pub fn lerp(&mut self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index] + next_index_weight
            * self.wave_table[next_index]
    }

}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.get_sample())
    }
}

impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

// doesn't work... I don't know why
// pub fn play(oscillator: WaveTableOscillator) {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let _result = stream_handle.play_raw(oscillator.convert_samples());
// }