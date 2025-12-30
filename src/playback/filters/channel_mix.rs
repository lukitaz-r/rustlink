use super::dsp::clamp_16_bit::clamp_16_bit;

pub struct ChannelMix {
    pub priority: u32,
    pub left_to_left: f32,
    pub left_to_right: f32,
    pub right_to_left: f32,
    pub right_to_right: f32,
}

impl ChannelMix {
    pub fn new() -> Self {
        Self {
            priority: 10,
            left_to_left: 1.0,
            left_to_right: 0.0,
            right_to_left: 0.0,
            right_to_right: 1.0,
        }
    }

    pub fn update(
        &mut self,
        left_to_left: Option<f32>,
        left_to_right: Option<f32>,
        right_to_left: Option<f32>,
        right_to_right: Option<f32>,
    ) {
        self.left_to_left = left_to_left.unwrap_or(1.0).clamp(0.0, 1.0);
        self.left_to_right = left_to_right.unwrap_or(0.0).clamp(0.0, 1.0);
        self.right_to_left = right_to_left.unwrap_or(0.0).clamp(0.0, 1.0);
        self.right_to_right = right_to_right.unwrap_or(1.0).clamp(0.0, 1.0);
    }

    pub fn process(&self, chunk: &mut [i16]) {
        if self.left_to_left == 1.0
            && self.left_to_right == 0.0
            && self.right_to_left == 0.0
            && self.right_to_right == 1.0
        {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let current_left_sample = frame[0] as f32;
            let current_right_sample = frame[1] as f32;

            let new_left_sample = current_left_sample * self.left_to_left
                + current_right_sample * self.right_to_left;
            let new_right_sample = current_left_sample * self.left_to_right
                + current_right_sample * self.right_to_right;

            frame[0] = clamp_16_bit(new_left_sample);
            frame[1] = clamp_16_bit(new_right_sample);
        }
    }
}
