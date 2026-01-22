use crate::playback::SAMPLE_RATE;
use super::dsp::clamp_16_bit::clamp_16_bit;
use crate::types::filters::BandSetting;
use std::f32::consts::PI;

const BAND_FREQUENCIES: [f32; 15] = [
    25.0, 40.0, 63.0, 100.0, 160.0, 250.0, 400.0, 630.0, 1000.0, 1600.0, 2500.0, 4000.0, 6300.0,
    10000.0, 16000.0,
];

const DEFAULT_Q: f32 = 1.0;

#[derive(Clone, Copy, Default)]
struct FilterState {
    xl1: f32,
    xl2: f32,
    yl1: f32,
    yl2: f32,
    xr1: f32,
    xr2: f32,
    yr1: f32,
    yr2: f32,
}

#[derive(Clone, Copy, Default)]
struct FilterCoefficients {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

pub struct Equalizer {
    pub priority: u32,
    filters_state: Vec<FilterState>,
    filters_coefficients: Vec<FilterCoefficients>,
}

impl Equalizer {
    pub fn new() -> Self {
        Self {
            priority: 10,
            filters_state: Vec::new(),
            filters_coefficients: Vec::new(),
        }
    }

    fn init_filters(&mut self) {
        self.filters_state.clear();
        self.filters_coefficients.clear();
        for _ in 0..BAND_FREQUENCIES.len() {
            self.filters_state.push(FilterState::default());
            self.filters_coefficients.push(FilterCoefficients {
                b0: 1.0,
                b1: 0.0,
                b2: 0.0,
                a1: 0.0,
                a2: 0.0,
            });
        }
    }

    fn update_band_coefficients(&mut self, band_index: usize, gain: f32) {
        if band_index >= BAND_FREQUENCIES.len() {
            return;
        }
        let freq = BAND_FREQUENCIES[band_index];
        let gain_db = gain * 12.0;
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega0 = (2.0 * PI * freq) / SAMPLE_RATE;
        let sin_omega0 = omega0.sin();
        let cos_omega0 = omega0.cos();

        let alpha = sin_omega0 / (2.0 * DEFAULT_Q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_omega0;
        let b2 = 1.0 - alpha * a;
        let mut a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_omega0;
        let a2 = 1.0 - alpha / a;

        if a0.abs() < 1e-12 {
            a0 = 1e-12;
        }

        self.filters_coefficients[band_index] = FilterCoefficients {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        };
    }

    pub fn update(&mut self, bands: &[BandSetting]) {
        if self.filters_state.is_empty() || self.filters_coefficients.is_empty() {
            self.init_filters();
        }

        // Reset
        for i in 0..BAND_FREQUENCIES.len() {
             self.filters_coefficients[i] = FilterCoefficients { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 };
             // We generally preserve state to avoid popping? 
             // JS code: `this.filtersState[i] = { ...0 }` resets state too.
             self.filters_state[i] = FilterState::default();
        }

        for band_setting in bands {
            if band_setting.band < BAND_FREQUENCIES.len() {
                self.update_band_coefficients(band_setting.band, band_setting.gain);
            }
        }
    }

    pub fn process(&mut self, chunk: &mut [i16]) {
        if self.filters_state.is_empty() {
            return;
        }

        for frame in chunk.chunks_exact_mut(2) {
            let mut current_left_sample = frame[0] as f32;
            let mut current_right_sample = frame[1] as f32;

            for b in 0..BAND_FREQUENCIES.len() {
                let coeffs = &self.filters_coefficients[b];
                let state = &mut self.filters_state[b];

                let new_left_sample = coeffs.b0 * current_left_sample
                    + coeffs.b1 * state.xl1
                    + coeffs.b2 * state.xl2
                    - coeffs.a1 * state.yl1
                    - coeffs.a2 * state.yl2;

                state.xl2 = state.xl1;
                state.xl1 = current_left_sample;
                state.yl2 = state.yl1;
                state.yl1 = new_left_sample;
                current_left_sample = new_left_sample;

                let new_right_sample = coeffs.b0 * current_right_sample
                    + coeffs.b1 * state.xr1
                    + coeffs.b2 * state.xr2
                    - coeffs.a1 * state.yr1
                    - coeffs.a2 * state.yr2;

                state.xr2 = state.xr1;
                state.xr1 = current_right_sample;
                state.yr2 = state.yr1;
                state.yr1 = new_right_sample;
                current_right_sample = new_right_sample;
            }

            frame[0] = clamp_16_bit(current_left_sample);
            frame[1] = clamp_16_bit(current_right_sample);
        }
    }
}
