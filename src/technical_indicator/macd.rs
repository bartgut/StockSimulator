use crate::technical_indicator::ema::Ema;

pub struct Macd {
    slow_period_ema: Ema,
    fast_period_ema: Ema,
    signal_period_ema: Ema
}

#[derive(Clone, Copy)]
pub struct MACDResult {
    pub macd_line: f32,
    pub signal_line: f32
}

impl Macd {
    pub fn new(slow_period:usize, fast_period: usize, signal_period: usize) -> Self {
        Self {
            slow_period_ema: Ema::new(slow_period),
            fast_period_ema: Ema::new(fast_period),
            signal_period_ema: Ema::new(signal_period)
        }
    }

    pub fn default() -> Self {
        Self {
            slow_period_ema: Ema::new(26),
            fast_period_ema: Ema::new(12),
            signal_period_ema: Ema::new(9)
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