use crate::playback::SAMPLE_RATE;
use super::dsp::clamp_16_bit::clamp_16_bit;
use super::dsp::delay::DelayLine;
use super::dsp::lfo::Lfo;
use super::dsp::waves::Waveform;

const MAX_DELAY_MS: f32 = 20.0;

pub struct Vibrato {
    pub priority: u32,
    lfo: Lfo,
    left_delay: DelayLine,
    right_delay: DelayLine,
    depth: f32,
}

impl Vibrato {
    pub fn new() -> Self {
        let buffer_size = (SAMPLE_RATE * MAX_DELAY_MS / 1000.0).ceil() as usize;
        Self {
            priority: 10,
            lfo: Lfo::new(Waveform::Sine, 0.0, 0.0),
            left_delay: DelayLine::new(buffer_size),
            right_delay: DelayLine::new(buffer_size),
            depth: 0.0,
        }
    }

    pub fn update(&mut self, frequency: Option<f32>, depth: Option<f32>) {
        let frequency = frequency.unwrap_or(0.0);
        let depth = depth.unwrap_or(0.0).clamp(0.0, 2.0);
        
        self.depth = depth; // Stored for process check
        self.lfo.update(frequency, depth);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.depth == 0.0 {
            self.left_delay.clear();
            self.right_delay.clear();
            return;
        }

        let max_delay_width = self.depth * (SAMPLE_RATE * 0.005);
        let center_delay = max_delay_width;

        for frame in chunk.chunks_exact_mut(2) {
            let lfo_value = self.lfo.get_value();
            
            let delay = center_delay + lfo_value * max_delay_width;

            let left_sample = frame[0];
            self.left_delay.write(left_sample);
            let delayed_left = self.left_delay.read(delay);
            frame[0] = clamp_16_bit(delayed_left as f32);
            
            let right_sample = frame[1];
            self.right_delay.write(right_sample);
            let delayed_right = self.right_delay.read(delay);
            frame[1] = clamp_16_bit(delayed_right as f32);
        }
    }
}
