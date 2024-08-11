use crate::technical_analysis::ema::Ema;

pub struct Atr {
    atr_ema: Ema
}

impl Atr {
    pub fn new(length: usize) -> Self {
        Self { atr_ema: Ema::new(length)  }
    }

    pub fn next(&mut self, today_high: f32, today_low: f32, yesterday_close: f32) -> f32 {
        let one = today_high - today_high;
        let two = today_high - yesterday_close;
        let three = yesterday_close - today_low;
        let true_range = one.max(two).max(three);
        self.atr_ema.next(true_range)
    }

    pub fn current(&self) -> f32 {
        return self.atr_ema.current()
    }
}