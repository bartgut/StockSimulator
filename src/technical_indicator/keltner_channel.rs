use crate::technical_indicator::atr::Atr;
use crate::technical_indicator::ema::Ema;

pub struct KeltnerChannel {
    channel_size: f32,
    ema: Ema,
    atr: Atr
}

#[derive(Clone)]
pub struct KeltnerChannelResult {
    pub ema: f32,
    pub upper_band: f32,
    pub lower_band: f32
}

impl Into<Vec<f32>> for KeltnerChannelResult {
    fn into(self) -> Vec<f32> {
        vec![self.lower_band, self.ema, self.upper_band]
    }
}

impl KeltnerChannel {
    pub fn new(length: usize, channel_size: f32) -> Self {
        Self {
            channel_size,
            ema: Ema::new(length),
            atr: Atr::new(length),
        }
    }

    pub fn current(&self) -> KeltnerChannelResult {
        KeltnerChannelResult {
            ema: self.ema.current(),
            upper_band: self.ema.current() + self.channel_size * self.atr.current(),
            lower_band: self.ema.current() - self.channel_size * self.atr.current()
        }
    }

    pub fn next(&mut self, price: f32, today_high: f32, today_low: f32, yesterday_close: f32) -> KeltnerChannelResult {
        let ema = self.ema.next(price);
        let atr = self.atr.next(today_high, today_low, yesterday_close);
        KeltnerChannelResult {
            ema,
            upper_band: ema + self.channel_size * atr,
            lower_band: ema - self.channel_size * atr
        }
    }

}