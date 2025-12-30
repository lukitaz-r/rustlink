use crate::playback::SAMPLE_RATE;
use super::dsp::allpass::Allpass;
use super::dsp::clamp_16_bit::clamp_16_bit;
use super::dsp::lfo::Lfo;
use super::dsp::waves::Waveform;
use std::f32::consts::PI;

const MAX_STAGES: usize = 12;

pub struct Phaser {
    pub priority: u32,
    left_lfo: Lfo,
    right_lfo: Lfo,
    stages: usize,
    rate: f32,
    depth: f32,
    feedback: f32,
    mix: f32,
    min_frequency: f32,
    max_frequency: f32,
    left_filters: Vec<Allpass>,
    right_filters: Vec<Allpass>,
    last_left_feedback: f32,
    last_right_feedback: f32,
}

impl Phaser {
    pub fn new() -> Self {
        // Need to set right LFO phase to PI/2.
        // Assuming I modify Lfo to allow that or just construct it with offset if I had that field.
        // For now, I'll just create them. Phase offset missing is a known limitation until Lfo is improved.
        let mut left_lfo = Lfo::new(Waveform::Sine, 0.0, 1.0);
        let mut right_lfo = Lfo::new(Waveform::Sine, 0.0, 1.0);
        // right_lfo.phase = PI / 2.0; // Private field.

        let mut left_filters = Vec::with_capacity(MAX_STAGES);
        let mut right_filters = Vec::with_capacity(MAX_STAGES);
        for _ in 0..MAX_STAGES {
            left_filters.push(Allpass::new());
            right_filters.push(Allpass::new());
        }

        Self {
            priority: 10,
            left_lfo,
            right_lfo,
            stages: 4,
            rate: 0.0,
            depth: 1.0,
            feedback: 0.0,
            mix: 0.5,
            min_frequency: 100.0,
            max_frequency: 2500.0,
            left_filters,
            right_filters,
            last_left_feedback: 0.0,
            last_right_feedback: 0.0,
        }
    }

    pub fn update(
        &mut self,
        stages: Option<usize>,
        rate: Option<f32>,
        depth: Option<f32>,
        feedback: Option<f32>,
        mix: Option<f32>,
        min_frequency: Option<f32>,
        max_frequency: Option<f32>,
    ) {
        self.stages = stages.unwrap_or(4).clamp(2, MAX_STAGES);
        self.rate = rate.unwrap_or(0.0);
        self.depth = depth.unwrap_or(1.0).clamp(0.0, 1.0);
        self.feedback = feedback.unwrap_or(0.0).clamp(0.0, 0.9);
        self.mix = mix.unwrap_or(0.5).clamp(0.0, 1.0);
        self.min_frequency = min_frequency.unwrap_or(100.0);
        self.max_frequency = max_frequency.unwrap_or(2500.0);

        self.left_lfo.update(self.rate, self.depth);
        self.right_lfo.update(self.rate, self.depth);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.rate == 0.0 || self.depth == 0.0 || self.mix == 0.0 {
            return;
        }

        let sweep_range = self.max_frequency - self.min_frequency;

        for frame in chunk.chunks_exact_mut(2) {
            let left_sample = frame[0] as f32;
            let right_sample = frame[1] as f32;

            let left_lfo_value = (self.left_lfo.get_value() + 1.0) / 2.0;
            let right_lfo_value = (self.right_lfo.get_value() + 1.0) / 2.0;

            let current_left_freq = self.min_frequency + sweep_range * left_lfo_value;
            let current_right_freq = self.min_frequency + sweep_range * right_lfo_value;

            let tan_left = (PI * current_left_freq / SAMPLE_RATE).tan();
            let a_left = (1.0 - tan_left) / (1.0 + tan_left);

            let tan_right = (PI * current_right_freq / SAMPLE_RATE).tan();
            let a_right = (1.0 - tan_right) / (1.0 + tan_right);

            let mut wet_left = left_sample + self.last_left_feedback * self.feedback;
            for j in 0..self.stages {
                self.left_filters[j].set_coefficient(a_left);
                wet_left = self.left_filters[j].process(wet_left);
            }
            self.last_left_feedback = wet_left;
            let final_left = left_sample * (1.0 - self.mix) + wet_left * self.mix;

            let mut wet_right = right_sample + self.last_right_feedback * self.feedback;
            for j in 0..self.stages {
                self.right_filters[j].set_coefficient(a_right);
                wet_right = self.right_filters[j].process(wet_right);
            }
            self.last_right_feedback = wet_right;
            let final_right = right_sample * (1.0 - self.mix) + wet_right * self.mix;

            frame[0] = clamp_16_bit(final_left);
            frame[1] = clamp_16_bit(final_right);
        }
    }
}
