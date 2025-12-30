pub fn clamp_16_bit(sample: f32) -> i16 {
    sample.round().clamp(-32768.0, 32767.0) as i16
}
