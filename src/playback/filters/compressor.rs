use crate::playback::SAMPLE_RATE;
use super::dsp::clamp_16_bit::clamp_16_bit;

fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

fn linear_to_db(linear: f32) -> f32 {
    if linear == 0.0 {
        -144.0
    } else {
        20.0 * linear.log10()
    }
}

pub struct Compressor {
    pub priority: u32,
    threshold: f32,
    ratio: f32,
    attack: f32,
    release: f32,
    gain: f32,
    attack_coeff: f32,
    release_coeff: f32,
    makeup_gain_linear: f32,
    envelope: f32,
}

impl Compressor {
    pub fn new() -> Self {
        Self {
            priority: 10,
            threshold: 0.0,
            ratio: 1.0,
            attack: 0.0,
            release: 0.0,
            gain: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            makeup_gain_linear: 1.0,
            envelope: 0.0,
        }
    }

    pub fn update(
        &mut self,
        threshold: Option<f32>,
        ratio: Option<f32>,
        attack: Option<f32>,
        release: Option<f32>,
        gain: Option<f32>,
    ) {
        self.threshold = threshold.unwrap_or(0.0);
        self.ratio = ratio.unwrap_or(1.0);
        self.attack = attack.unwrap_or(0.0);
        self.release = release.unwrap_or(0.0);
        self.gain = gain.unwrap_or(0.0);

        self.attack_coeff = if self.attack > 0.0 {
            (-1.0 / ((self.attack / 1000.0) * SAMPLE_RATE)).exp()
        } else {
            0.0
        };

        self.release_coeff = if self.release > 0.0 {
            (-1.0 / ((self.release / 1000.0) * SAMPLE_RATE)).exp()
        } else {
            0.0
        };

        self.makeup_gain_linear = db_to_linear(self.gain);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.threshold == 0.0 && self.ratio == 1.0 && self.gain == 0.0 {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let left_sample = frame[0] as f32;
            let right_sample = frame[1] as f32;

            let peak = left_sample.abs().max(right_sample.abs());

            if peak > self.envelope {
                self.envelope = self.attack_coeff * self.envelope + (1.0 - self.attack_coeff) * peak;
            } else {
                self.envelope = self.release_coeff * self.envelope + (1.0 - self.release_coeff) * peak;
            }

            let envelope_db = linear_to_db(self.envelope / 32767.0);
            let mut gain_reduction_db = 0.0;

            if self.ratio > 1.0 && envelope_db > self.threshold {
                gain_reduction_db = (self.threshold - envelope_db) * (1.0 - 1.0 / self.ratio);
            }

            let target_gain_linear = db_to_linear(gain_reduction_db) * self.makeup_gain_linear;

            let new_left = left_sample * target_gain_linear;
            let new_right = right_sample * target_gain_linear;

            frame[0] = clamp_16_bit(new_left);
            frame[1] = clamp_16_bit(new_right);
        }
    }
}
