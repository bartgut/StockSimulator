use crate::atr::Atr;
use crate::ema::Ema;

pub struct KeltnerChannel {
    ema: Ema,
    atr: Atr
}

pub struct KeltnerChannelResult {
    pub ema: f32,
    pub upper_band: f32,
    pub lower_band: f32
}

impl KeltnerChannel {
    pub fn new(length: usize) -> Self {
        Self {
            ema: Ema::new(length),
            atr: Atr::new(length)
        }
    }

    pub fn next(&mut self, price: f32, today_high: f32, today_low: f32, yesterday_close: f32)  -> KeltnerChannelResult {
        let ema = self.ema.next(price);
        let atr = self.atr.next(today_high, today_low, yesterday_close);
        KeltnerChannelResult {
            ema,
            upper_band: ema + 2.0*atr,
            lower_band: ema - 2.0*atr
        }
    }

}