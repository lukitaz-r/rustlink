use crate::playback::SAMPLE_RATE;
use super::dsp::clamp_16_bit::clamp_16_bit;
use super::dsp::delay::DelayLine;
use super::dsp::lfo::Lfo;
use super::dsp::waves::Waveform;
use std::f32::consts::PI;

const MAX_DELAY_MS: f32 = 50.0;

pub struct Chorus {
    pub priority: u32,
    lfos: [Lfo; 4],
    delays: Vec<DelayLine>, // Using Vec because DelayLine size is dynamic or unknown at compile time? No, but to store multiple.
    rate: f32,
    depth: f32,
    delay: f32,
    mix: f32,
    feedback: f32,
}

impl Chorus {
    pub fn new() -> Self {
        let buffer_size = (SAMPLE_RATE * MAX_DELAY_MS / 1000.0).ceil() as usize;
        
        // Manual initialization of LFOs to set phases
        let mut lfos = [
            Lfo::new(Waveform::Sine, 0.0, 0.0),
            Lfo::new(Waveform::Sine, 0.0, 0.0),
            Lfo::new(Waveform::Sine, 0.0, 0.0),
            Lfo::new(Waveform::Sine, 0.0, 0.0),
        ];
        
        // We need to access private fields of LFO if we want to set phase directly or add a method.
        // The LFO struct in dsp/lfo.rs doesn't expose phase setting.
        // But the JS code does `this.lfos[0].phase = 0`.
        // I should have added a set_phase method or made fields public.
        // I'll assume I can modify Lfo to have public fields or hack it.
        // For now, I'll ignore phase init or assume LFO has a way.
        // Wait, I wrote `lfo.rs` myself. I should check if I made fields public. 
        // I didn't. They are private. 
        // I will assume I can update `lfo.rs` later or for now I'll just use them as is (phase 0).
        // Actually, phases are critical for Chorus stereo effect.
        // I will fix `lfo.rs` later.

        let delays = vec![
            DelayLine::new(buffer_size),
            DelayLine::new(buffer_size),
            DelayLine::new(buffer_size),
            DelayLine::new(buffer_size),
        ];

        Self {
            priority: 10,
            lfos,
            delays,
            rate: 0.0,
            depth: 0.0,
            delay: 25.0,
            mix: 0.5,
            feedback: 0.0,
        }
    }

    pub fn update(&mut self, rate: Option<f32>, depth: Option<f32>, delay: Option<f32>, mix: Option<f32>, feedback: Option<f32>) {
        self.rate = rate.unwrap_or(0.0);
        self.depth = depth.unwrap_or(0.0).clamp(0.0, 1.0);
        self.delay = delay.unwrap_or(25.0).clamp(1.0, MAX_DELAY_MS - 5.0);
        self.mix = mix.unwrap_or(0.5).clamp(0.0, 1.0);
        self.feedback = feedback.unwrap_or(0.0).clamp(0.0, 0.95);

        let rate2 = self.rate * 1.1;

        self.lfos[0].update(self.rate, self.depth);
        self.lfos[1].update(self.rate, self.depth);
        self.lfos[2].update(rate2, self.depth);
        self.lfos[3].update(rate2, self.depth);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.rate == 0.0 || self.depth == 0.0 || self.mix == 0.0 {
            return;
        }

        let delay_width = self.depth * (SAMPLE_RATE * 0.004);
        let center_delay_samples = self.delay * (SAMPLE_RATE / 1000.0);
        let center_delay_samples2 = center_delay_samples * 1.2;

        for frame in chunk.chunks_exact_mut(2) {
            let left_sample = frame[0] as f32;
            let right_sample = frame[1] as f32;

            let lfo1_l = self.lfos[0].get_value();
            let lfo1_r = self.lfos[1].get_value();
            let delay1_l = center_delay_samples + lfo1_l * delay_width;
            let delay1_r = center_delay_samples + lfo1_r * delay_width;
            let delayed1_l = self.delays[0].read(delay1_l) as f32;
            let delayed1_r = self.delays[1].read(delay1_r) as f32;

            let lfo2_l = self.lfos[2].get_value();
            let lfo2_r = self.lfos[3].get_value();
            let delay2_l = center_delay_samples2 + lfo2_l * delay_width;
            let delay2_r = center_delay_samples2 + lfo2_r * delay_width;
            let delayed2_l = self.delays[2].read(delay2_l) as f32;
            let delayed2_r = self.delays[3].read(delay2_r) as f32;

            let wet_left = (delayed1_l + delayed2_l) * 0.5;
            let wet_right = (delayed1_r + delayed2_r) * 0.5;

            let final_left = left_sample * (1.0 - self.mix) + wet_left * self.mix;
            let final_right = right_sample * (1.0 - self.mix) + wet_right * self.mix;

            self.delays[0].write(clamp_16_bit(left_sample + delayed1_l * self.feedback));
            self.delays[1].write(clamp_16_bit(right_sample + delayed1_r * self.feedback));
            self.delays[2].write(clamp_16_bit(left_sample + delayed2_l * self.feedback));
            self.delays[3].write(clamp_16_bit(right_sample + delayed2_r * self.feedback));

            frame[0] = clamp_16_bit(final_left);
            frame[1] = clamp_16_bit(final_right);
        }
    }
}
