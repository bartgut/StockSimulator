pub struct Ema {
    length: usize,
    current_ema: f32,
}

impl Ema {
    pub fn new(length: usize) -> Self {
        Self { length, current_ema: 0.0f32 }
    }

    pub fn next(&mut self, price: f32) -> f32 {
        let new_ema = price * self.k_param() + self.current_ema * (1.0f32 - self.k_param());
        self.current_ema = new_ema;
        return self.current_ema;
    }

    fn k_param(&self) -> f32 {
        2.0f32 / ((self.length as f32) + 1.0f32)
    }
}