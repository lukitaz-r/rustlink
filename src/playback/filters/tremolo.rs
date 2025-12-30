use super::dsp::clamp_16_bit::clamp_16_bit;
use super::dsp::lfo::Lfo;
use super::dsp::waves::Waveform;

pub struct Tremolo {
    pub priority: u32,
    lfo: Lfo,
}

impl Tremolo {
    pub fn new() -> Self {
        Self {
            priority: 10,
            lfo: Lfo::new(Waveform::Sine, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, frequency: Option<f32>, depth: Option<f32>) {
        let frequency = frequency.unwrap_or(0.0);
        let depth = depth.unwrap_or(0.0).clamp(0.0, 1.0);

        self.lfo.update(frequency, depth);
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        // Need to check depth/freq.
        // Assuming LFO works if updated.
        
        for frame in chunk.chunks_exact_mut(2) {
             // JS Tremolo loop: i += 2. Single loop over all samples?
             // "for (let i = 0; i < chunk.length; i += 2)"
             // "chunk.readInt16LE(i)" -> This is every sample, L then R.
             // Wait, JS reads i, i+2... no, i+=2 means 2 bytes.
             // So it iterates over every sample individually.
             // "const multiplier = this.lfo.process()"
             // It calls lfo.process() for EVERY sample?
             // Yes. The LFO advances per sample.
             // Note: LFO logic uses phase += (2PI * freq) / SAMPLE_RATE.
             // So it's correct to call it per sample if LFO is intended to run at sample rate.
             // In JS: "const newSample = sample * multiplier"
             
             let left_sample = frame[0] as f32;
             let left_multiplier = self.lfo.process();
             frame[0] = clamp_16_bit(left_sample * left_multiplier);
             
             let right_sample = frame[1] as f32;
             let right_multiplier = self.lfo.process();
             frame[1] = clamp_16_bit(right_sample * right_multiplier);
        }
    }
}
