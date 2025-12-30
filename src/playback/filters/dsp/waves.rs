use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, Default)]
pub enum Waveform {
    #[default]
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Waveform {
    pub fn apply(&self, phase: f32) -> f32 {
        match self {
            Waveform::Sine => phase.sin(),
            Waveform::Square => {
                if (phase % (2.0 * PI)) < PI {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Sawtooth => (phase % (2.0 * PI)) / PI - 1.0,
            Waveform::Triangle => {
                let x = (phase % (2.0 * PI)) / (2.0 * PI);
                2.0 * if x < 0.5 { 2.0 * x } else { 2.0 - 2.0 * x } - 1.0
            }
        }
    }
}
