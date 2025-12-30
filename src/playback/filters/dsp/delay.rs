pub struct DelayLine {
    buffer: Vec<i16>,
    size: usize,
    write_index: usize,
}

impl DelayLine {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size],
            size,
            write_index: 0,
        }
    }

    pub fn write(&mut self, sample: i16) {
        self.buffer[self.write_index] = sample;
        self.write_index = (self.write_index + 1) % self.size;
    }

    pub fn read(&self, delay_in_samples: f32) -> i16 {
        let safe_delay = delay_in_samples
            .floor()
            .max(0.0)
            .min((self.size - 1) as f32) as usize;
        let read_index = (self.write_index + self.size - safe_delay) % self.size;

        self.buffer[read_index]
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
