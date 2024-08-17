use crate::technical_indicator::ema::Ema;

pub struct Macd {
    fast_period_ema: Ema,
    slow_period_ema: Ema,
    signal_period_ema: Ema
}

pub struct MACDResult {
    pub macd_line: f32,
    pub signal_line: f32
}

impl Macd {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            fast_period_ema: Ema::new(fast_period),
            slow_period_ema: Ema::new(slow_period),
            signal_period_ema: Ema::new(signal_period)
        }
    }

    pub fn next(&mut self, price: f32) -> MACDResult {
        let fast_ema = self.fast_period_ema.next(price);
        let slow_ema = self.slow_period_ema.next(price);
        let macd_line = fast_ema - slow_ema;
        let signal_line = self.signal_period_ema.next(macd_line);
        MACDResult {
            macd_line,
            signal_line
        }
    }
}