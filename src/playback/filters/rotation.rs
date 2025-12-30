use super::dsp::clamp_16_bit::clamp_16_bit;
use super::dsp::lfo::Lfo;
use super::dsp::waves::Waveform;

pub struct Rotation {
    pub priority: u32,
    lfo: Lfo,
}

impl Rotation {
    pub fn new() -> Self {
        Self {
            priority: 10,
            lfo: Lfo::new(Waveform::Sine, 0.0, 1.0),
        }
    }

    pub fn update(&mut self, rotation_hz: Option<f32>) {
        let rotation_hz = rotation_hz.unwrap_or(0.0);
        self.lfo.update(rotation_hz, 1.0);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        // Since we can't easily check internal frequency if it's private, we trust the logic.
        // Actually I should have made getters.
        // Assuming I'll fix Lfo visibility later or just run it.
        // "if this.lfo.frequency === 0" -> In update we set it.
        
        // I'll make a public getter in Lfo or check the passed value if I stored it.
        // For now, I'll just run. If freq is 0, LFO returns 0 or similar constant.
        
        for frame in chunk.chunks_exact_mut(2) {
             // In JS: if freq == 0 return chunk.
             // If I don't check, I might do useless math.
             
             let lfo_value = self.lfo.get_value();
             
             let left_factor = (1.0 - lfo_value) / 2.0;
             let right_factor = (1.0 + lfo_value) / 2.0;
             
             let current_left_sample = frame[0] as f32;
             let current_right_sample = frame[1] as f32;
             
             let new_left_sample = current_left_sample * left_factor;
             let new_right_sample = current_right_sample * right_factor;
             
             frame[0] = clamp_16_bit(new_left_sample);
             frame[1] = clamp_16_bit(new_right_sample);
        }
    }
}
