pub struct Allpass {
    x1: f32,
    y1: f32,
    a: f32,
}

impl Allpass {
    pub fn new() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            a: 0.0,
        }
    }

    pub fn set_coefficient(&mut self, a: f32) {
        self.a = a.clamp(-0.999, 0.999);
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let output = self.a * sample + self.x1 - self.a * self.y1;

        self.x1 = sample;
        self.y1 = output;

        output
    }
}
