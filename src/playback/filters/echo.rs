use crate::playback::SAMPLE_RATE;
use super::dsp::clamp_16_bit::clamp_16_bit;
use super::dsp::delay::DelayLine;

const MAX_DELAY_S: f32 = 5.0;

pub struct Echo {
    pub priority: u32,
    delay: f32,
    feedback: f32,
    mix: f32,
    delay_time_samples: f32,
    left_delay: DelayLine,
    right_delay: DelayLine,
}

impl Echo {
    pub fn new() -> Self {
        let buffer_size = (SAMPLE_RATE * MAX_DELAY_S).ceil() as usize;
        Self {
            priority: 10,
            delay: 0.0,
            feedback: 0.0,
            mix: 0.0,
            delay_time_samples: 0.0,
            left_delay: DelayLine::new(buffer_size),
            right_delay: DelayLine::new(buffer_size),
        }
    }

    pub fn update(&mut self, delay: Option<f32>, feedback: Option<f32>, mix: Option<f32>) {
        self.delay = delay.unwrap_or(0.0).clamp(0.0, MAX_DELAY_S * 1000.0);
        self.feedback = feedback.unwrap_or(0.0).clamp(0.0, 1.0);
        self.mix = mix.unwrap_or(0.0).clamp(0.0, 1.0);

        self.delay_time_samples = self.delay * (SAMPLE_RATE / 1000.0);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.delay == 0.0 || self.mix == 0.0 {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let left_sample = frame[0] as f32;
            let right_sample = frame[1] as f32;

            let delayed_left = self.left_delay.read(self.delay_time_samples) as f32;
            let delayed_right = self.right_delay.read(self.delay_time_samples) as f32;

            self.left_delay.write(clamp_16_bit(left_sample + delayed_left * self.feedback));
            self.right_delay.write(clamp_16_bit(right_sample + delayed_right * self.feedback));

            let new_left = left_sample * (1.0 - self.mix) + delayed_left * self.mix;
            let new_right = right_sample * (1.0 - self.mix) + delayed_right * self.mix;

            frame[0] = clamp_16_bit(new_left);
            frame[1] = clamp_16_bit(new_right);
        }
    }
}
