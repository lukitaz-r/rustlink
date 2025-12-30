use super::dsp::clamp_16_bit::clamp_16_bit;

fn cubic_interpolate(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;

    0.5 * (2.0 * p1
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

pub struct Timescale {
    pub priority: u32,
    speed: f32,
    pitch: f32,
    rate: f32,
    final_rate: f32,
    input_buffer: Vec<i16>,
}

impl Timescale {
    pub fn new() -> Self {
        Self {
            priority: 1,
            speed: 1.0,
            pitch: 1.0,
            rate: 1.0,
            final_rate: 1.0,
            input_buffer: Vec::new(),
        }
    }

    pub fn update(&mut self, speed: Option<f32>, pitch: Option<f32>, rate: Option<f32>) {
        self.speed = speed.unwrap_or(1.0);
        self.pitch = pitch.unwrap_or(1.0);
        self.rate = rate.unwrap_or(1.0);

        self.final_rate = self.speed * self.pitch * self.rate;
    }

    // Takes input slice, returns processed Vec.
    pub fn process(&mut self, chunk: &[i16]) -> Vec<i16> {
        if (self.final_rate - 1.0).abs() < f32::EPSILON {
             // Pass through if rate is 1.0. 
             // But we might have leftover input_buffer? 
             // JS: "if (this.finalRate === 1.0) { return chunk }"
             // It implies ignoring input_buffer if rate is 1?
             // Or maybe input_buffer is empty if rate was 1?
             // If rate changes dynamically, we should probably drain buffer.
             // But following JS exactly:
             return chunk.to_vec();
        }
        
        if self.final_rate == 0.0 {
            return Vec::new();
        }

        self.input_buffer.extend_from_slice(chunk);

        if self.input_buffer.len() < 16 { // JS: < 16 bytes -> 8 samples? No, readInt16LE takes 2 bytes. 16 bytes = 8 samples (4 stereo frames).
             // JS uses Buffer.length which is bytes.
             // Rust Vec<i16>. So 8 i16s = 16 bytes.
             if self.input_buffer.len() < 8 {
                 return Vec::new();
             }
        }
        
        // JS: outputLength = Math.floor(this.inputBuffer.length / this.finalRate)
        // inputBuffer.length is BYTES.
        // outputLength is BYTES.
        // So in samples (i16):
        let input_samples = self.input_buffer.len(); // i16 count
        // input_bytes = input_samples * 2
        // output_bytes = floor(input_bytes / rate)
        // output_samples = output_bytes / 2
        
        let output_samples_count = ((input_samples * 2) as f32 / self.final_rate).floor() as usize / 2;
        
        // JS: finalOutputLength = outputLength - (outputLength % 4)
        // 4 bytes = 2 i16s = 1 stereo frame.
        // So ensure output_samples is multiple of 2.
        let final_output_samples = output_samples_count - (output_samples_count % 2);
        
        let mut output_buffer = Vec::with_capacity(final_output_samples);

        let mut output_pos = 0; // in samples (i16 index)
        
        // JS loop uses outputPos (bytes).
        // Rust loop uses output_pos (i16 index).
        // 4 bytes = 2 samples.
        
        while output_pos < final_output_samples {
             // JS: inputFrame = (outputPos / 4) * this.finalRate
             // outputPos / 4 is the frame index (stereo pair).
             // output_pos / 2 is the frame index.
             
             let input_frame = (output_pos / 2) as f32 * self.final_rate;
             let i1 = input_frame.floor() as isize;
             let frac = input_frame - i1 as f32;
             
             // Stereo frames. i1 is frame index.
             // Sample index = i1 * 2.
             
             let p0_idx = i1 - 1;
             let p1_idx = i1;
             let p2_idx = i1 + 1;
             let p3_idx = i1 + 2;
             
             // Check boundary: (p3_idx + 1) * 4 > buffer.length
             // (p3_idx + 1) frames > buffer frames?
             // buffer frames = len / 2.
             // p3_idx is frame index.
             
             if (p3_idx + 1) * 2 > self.input_buffer.len() as isize {
                 break;
             }
             
             fn get_sample(buf: &[i16], frame_idx: isize, offset: usize) -> f32 {
                 // frame_idx can be negative (p0_idx < 0)
                 // JS: if p0_idx < 0 ? read(p1_idx) : read(p0_idx)
                 let idx = if frame_idx < 0 {
                     // p1_idx * 4 + offset. p1_idx = i1 = 0 if p0 < 0?
                     // Usually p1_idx is valid if p0 is -1.
                     (frame_idx + 1) * 2 + offset as isize
                 } else {
                     frame_idx * 2 + offset as isize
                 };
                 buf[idx as usize] as f32
             }
             
             // Left
             let p0_l = get_sample(&self.input_buffer, p0_idx, 0);
             let p1_l = get_sample(&self.input_buffer, p1_idx, 0);
             let p2_l = get_sample(&self.input_buffer, p2_idx, 0);
             let p3_l = get_sample(&self.input_buffer, p3_idx, 0);
             let out_l = cubic_interpolate(p0_l, p1_l, p2_l, p3_l, frac);
             output_buffer.push(clamp_16_bit(out_l));
             
             // Right
             let p0_r = get_sample(&self.input_buffer, p0_idx, 1);
             let p1_r = get_sample(&self.input_buffer, p1_idx, 1);
             let p2_r = get_sample(&self.input_buffer, p2_idx, 1);
             let p3_r = get_sample(&self.input_buffer, p3_idx, 1);
             let out_r = cubic_interpolate(p0_r, p1_r, p2_r, p3_r, frac);
             output_buffer.push(clamp_16_bit(out_r));
             
             output_pos += 2;
        }
        
        // Consumed input bytes: Math.floor((outputPos / 4) * this.finalRate) * 4
        // outputPos (bytes). output_pos (samples).
        // frame_count = output_pos / 2
        let consumed_frames = ((output_pos / 2) as f32 * self.final_rate).floor() as usize;
        let consumed_samples = consumed_frames * 2;
        
        // Remove consumed samples
        if consumed_samples < self.input_buffer.len() {
            self.input_buffer.drain(0..consumed_samples);
        } else {
            self.input_buffer.clear();
        }
        
        output_buffer
    }
}
