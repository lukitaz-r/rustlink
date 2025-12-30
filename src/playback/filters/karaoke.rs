use crate::playback::SAMPLE_RATE;
use super::dsp::clamp_16_bit::clamp_16_bit;
use std::f32::consts::PI;

pub struct Karaoke {
    pub priority: u32,
    level: f32,
    mono_level: f32,
    filter_band: f32,
    filter_width: f32,
    xl1: f32,
    xl2: f32,
    yl1: f32,
    yl2: f32,
    xr1: f32,
    xr2: f32,
    yr1: f32,
    yr2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    a0: f32,
    a1: f32,
    a2: f32,
}

impl Karaoke {
    pub fn new() -> Self {
        Self {
            priority: 10,
            level: 0.0,
            mono_level: 0.0,
            filter_band: 0.0,
            filter_width: 0.0,
            xl1: 0.0,
            xl2: 0.0,
            yl1: 0.0,
            yl2: 0.0,
            xr1: 0.0,
            xr2: 0.0,
            yr1: 0.0,
            yr2: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
        }
    }

    fn update_coefficients(&mut self) {
        if self.filter_band == 0.0 || self.filter_width == 0.0 {
            self.b0 = 1.0;
            self.b1 = 0.0;
            self.b2 = 0.0;
            self.a1 = 0.0;
            self.a2 = 0.0;
            return;
        }

        let omega0 = (2.0 * PI * self.filter_band) / SAMPLE_RATE;
        let q = self.filter_band / (self.filter_width * (1.0 - self.level + 0.001));
        let alpha = omega0.sin() / (2.0 * q);
        let cos_omega0 = omega0.cos();

        self.b0 = 1.0;
        self.b1 = -2.0 * cos_omega0;
        self.b2 = 1.0;
        self.a0 = 1.0 + alpha;

        if self.a0.abs() < 1e-9 {
            self.a0 = 1e-9;
        }

        self.a1 = -2.0 * cos_omega0;
        self.a2 = 1.0 - alpha;

        self.b0 /= self.a0;
        self.b1 /= self.a0;
        self.b2 /= self.a0;
        self.a1 /= self.a0;
        self.a2 /= self.a0;
        self.a0 = 1.0;
    }

    pub fn update(
        &mut self,
        level: Option<f32>,
        mono_level: Option<f32>,
        filter_band: Option<f32>,
        filter_width: Option<f32>,
    ) {
        self.level = level.unwrap_or(0.0).clamp(0.0, 1.0);
        self.mono_level = mono_level.unwrap_or(0.0).clamp(0.0, 1.0);
        self.filter_band = filter_band.unwrap_or(0.0);
        self.filter_width = filter_width.unwrap_or(0.0);

        self.update_coefficients();

        self.xl1 = 0.0;
        self.xl2 = 0.0;
        self.yl1 = 0.0;
        self.yl2 = 0.0;
        self.xr1 = 0.0;
        self.xr2 = 0.0;
        self.yr1 = 0.0;
        self.yr2 = 0.0;
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.level == 0.0 && self.mono_level == 0.0 {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let mut current_left_sample = frame[0] as f32;
            let mut current_right_sample = frame[1] as f32;

            if self.mono_level > 0.0 {
                let mono = (current_left_sample + current_right_sample) / 2.0;
                current_left_sample = current_left_sample - mono * self.mono_level;
                current_right_sample = current_right_sample - mono * self.mono_level;
            }

            if self.level > 0.0 && self.filter_band != 0.0 && self.filter_width != 0.0 {
                let new_left_sample = self.b0 * current_left_sample
                    + self.b1 * self.xl1
                    + self.b2 * self.xl2
                    - self.a1 * self.yl1
                    - self.a2 * self.yl2;
                self.xl2 = self.xl1;
                self.xl1 = current_left_sample;
                self.yl2 = self.yl1;
                self.yl1 = new_left_sample;
                current_left_sample = new_left_sample;

                let new_right_sample = self.b0 * current_right_sample
                    + self.b1 * self.xr1
                    + self.b2 * self.xr2
                    - self.a1 * self.yr1
                    - self.a2 * self.yr2;
                self.xr2 = self.xr1;
                self.xr1 = current_right_sample;
                self.yr2 = self.yr1;
                self.yr1 = new_right_sample;
                current_right_sample = new_right_sample;
            }

            frame[0] = clamp_16_bit(current_left_sample);
            frame[1] = clamp_16_bit(current_right_sample);
        }
    }
}
