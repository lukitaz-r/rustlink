use super::dsp::clamp_16_bit::clamp_16_bit;

pub struct Highpass {
    pub priority: u32,
    smoothing: f32,
    smoothing_factor: f32,
    prev_left_input: f32,
    prev_right_input: f32,
    prev_left_lowpass_output: f32,
    prev_right_lowpass_output: f32,
}

impl Highpass {
    pub fn new() -> Self {
        Self {
            priority: 10,
            smoothing: 0.0,
            smoothing_factor: 0.0,
            prev_left_input: 0.0,
            prev_right_input: 0.0,
            prev_left_lowpass_output: 0.0,
            prev_right_lowpass_output: 0.0,
        }
    }

    pub fn update(&mut self, smoothing: Option<f32>) {
        let smoothing = smoothing.unwrap_or(0.0);
        if smoothing > 1.0 {
            self.smoothing = smoothing;
            self.smoothing_factor = 1.0 / smoothing;
        } else {
            self.smoothing = 0.0;
            self.smoothing_factor = 0.0;
        }
        self.prev_left_input = 0.0;
        self.prev_right_input = 0.0;
        self.prev_left_lowpass_output = 0.0;
        self.prev_right_lowpass_output = 0.0;
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.smoothing <= 1.0 {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let current_left_sample = frame[0] as f32;
            let current_right_sample = frame[1] as f32;

            let new_left_lowpass_output = self.prev_left_lowpass_output
                + self.smoothing_factor * (current_left_sample - self.prev_left_lowpass_output);
            self.prev_left_lowpass_output = new_left_lowpass_output;

            let new_left_sample = current_left_sample - new_left_lowpass_output;
            frame[0] = clamp_16_bit(new_left_sample);

            let new_right_lowpass_output = self.prev_right_lowpass_output
                + self.smoothing_factor * (current_right_sample - self.prev_right_lowpass_output);
            self.prev_right_lowpass_output = new_right_lowpass_output;

            let new_right_sample = current_right_sample - new_right_lowpass_output;
            frame[1] = clamp_16_bit(new_right_sample);
        }
    }
}
