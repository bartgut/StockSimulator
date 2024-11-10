use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::ema::Ema;
use crate::technical_indicator::macd::{Macd, MACDResult};

pub struct MACDStrategy {
    macd: Macd
}

impl MACDStrategy {

    pub fn new(slow_period:usize, fast_period: usize, signal_period: usize) -> Self {
        Self {
            macd: Macd::new(slow_period, fast_period, signal_period)
        }
    }

    pub fn default() -> Self {
        MACDStrategy {
            macd: Macd::default()
        }
    }
}
impl InvestingStrategy<MACDResult> for MACDStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, _: &Option<StockPriceInfo>) -> MACDResult {
        self.macd.next(stock_price_info.close)
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &MACDResult) -> Option<f32> {
        if indicator.macd_line > indicator.signal_line {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &MACDResult) -> Option<f32> {
        if indicator.macd_line < indicator.signal_line {
            Some(stock_price_info.close)
        } else {
            None
        }
    }
}