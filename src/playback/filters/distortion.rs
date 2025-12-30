use super::dsp::clamp_16_bit::clamp_16_bit;
use std::f32::consts::PI;

const MAX_INT_16: f32 = 32767.0;

pub struct Distortion {
    pub priority: u32,
    sin_offset: f32,
    sin_scale: f32,
    cos_offset: f32,
    cos_scale: f32,
    tan_offset: f32,
    tan_scale: f32,
    offset: f32,
    scale: f32,
}

impl Distortion {
    pub fn new() -> Self {
        Self {
            priority: 10,
            sin_offset: 0.0,
            sin_scale: 0.0,
            cos_offset: 0.0,
            cos_scale: 0.0,
            tan_offset: 0.0,
            tan_scale: 0.0,
            offset: 0.0,
            scale: 1.0,
        }
    }

    pub fn update(
        &mut self,
        sin_offset: Option<f32>,
        sin_scale: Option<f32>,
        cos_offset: Option<f32>,
        cos_scale: Option<f32>,
        tan_offset: Option<f32>,
        tan_scale: Option<f32>,
        offset: Option<f32>,
        scale: Option<f32>,
    ) {
        self.sin_offset = sin_offset.unwrap_or(0.0);
        self.sin_scale = sin_scale.unwrap_or(0.0);
        self.cos_offset = cos_offset.unwrap_or(0.0);
        self.cos_scale = cos_scale.unwrap_or(0.0);
        self.tan_offset = tan_offset.unwrap_or(0.0);
        self.tan_scale = tan_scale.unwrap_or(0.0);
        self.offset = offset.unwrap_or(0.0);
        self.scale = scale.unwrap_or(1.0);
    }

    pub fn process(&self, chunk: &mut [i16]) {
        if self.sin_scale == 0.0
            && self.cos_scale == 0.0
            && self.tan_scale == 0.0
            && self.offset == 0.0
            && self.scale == 1.0
        {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let current_left_sample = frame[0] as f32;
            let current_right_sample = frame[1] as f32;

            let normalized_left = current_left_sample / MAX_INT_16;
            let normalized_right = current_right_sample / MAX_INT_16;

            let mut distorted_left = 0.0;
            let mut distorted_right = 0.0;

            if self.sin_scale != 0.0 {
                distorted_left += (normalized_left * self.sin_scale + self.sin_offset).sin();
                distorted_right += (normalized_right * self.sin_scale + self.sin_offset).sin();
            }

            if self.cos_scale != 0.0 {
                distorted_left += (normalized_left * self.cos_scale + self.cos_offset).cos();
                distorted_right += (normalized_right * self.cos_scale + self.cos_offset).cos();
            }

            if self.tan_scale != 0.0 {
                let tan_input_left = (normalized_left * self.tan_scale + self.tan_offset).clamp(
                    -PI / 2.0 + 0.01,
                    PI / 2.0 - 0.01,
                );
                let tan_input_right = (normalized_right * self.tan_scale + self.tan_offset).clamp(
                    -PI / 2.0 + 0.01,
                    PI / 2.0 - 0.01,
                );

                distorted_left += tan_input_left.tan();
                distorted_right += tan_input_right.tan();
            }

            distorted_left = (distorted_left * self.scale + self.offset) * MAX_INT_16;
            distorted_right = (distorted_right * self.scale + self.offset) * MAX_INT_16;

            frame[0] = clamp_16_bit(distorted_left);
            frame[1] = clamp_16_bit(distorted_right);
        }
    }
}
