use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::rsi::{Rsi, RsiResult};

pub struct RsiStrategy {
    rsi: Rsi,
    lower_band: f32,
    higher_band: f32,
}

impl RsiStrategy {
    pub fn new(length: usize, lower_band: f32, higher_band: f32) -> Self {
        RsiStrategy {
            rsi: Rsi::new(length),
            lower_band,
            higher_band
        }
    }
}

impl InvestingStrategy<RsiResult> for RsiStrategy {

    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> RsiResult {
        self.rsi.next(stock_price_info.close)
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &RsiResult) -> Option<f32> {
        if indicator.rsi_line < self.lower_band {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &RsiResult) -> Option<f32> {
        if indicator.rsi_line > self.higher_band {
            Some(stock_price_info.close)
        } else {
            None
        }
    }
}