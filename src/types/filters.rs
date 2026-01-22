use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BandSetting {
    pub band: usize,
    pub gain: f32,
}
