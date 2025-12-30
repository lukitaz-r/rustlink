use super::waves::Waveform;
use std::f32::consts::PI;

const SAMPLE_RATE: f32 = 48000.0;

pub struct Lfo {
    phase: f32,
    waveform: Waveform,
    frequency: f32,
    depth: f32,
}

impl Lfo {
    pub fn new(waveform: Waveform, frequency: f32, depth: f32) -> Self {
        Self {
            phase: 0.0,
            waveform,
            frequency,
            depth,
        }
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn update(&mut self, frequency: f32, depth: f32) {
        self.frequency = frequency;
        self.depth = depth;
    }

    pub fn get_value(&mut self) -> f32 {
        if self.frequency == 0.0 {
            return 0.0;
        }
        let value = self.waveform.apply(self.phase);
        self.phase += (2.0 * PI * self.frequency) / SAMPLE_RATE;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        return value;
    }

    pub fn process(&mut self) -> f32 {
        if self.depth == 0.0 || self.frequency == 0.0 {
            return 1.0;
        }
        let lfo_value = self.get_value();
        let normalized_lfo = (lfo_value + 1.0) / 2.0;

        1.0 - self.depth * normalized_lfo
    }
}
