use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_analysis::keltner_channel::{KeltnerChannel, KeltnerChannelResult};

impl InvestingStrategy<KeltnerChannelResult> for KeltnerChannel {
    fn calculation(&mut self, today: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> KeltnerChannelResult {
        self.next(today.close, today.high, today.low, yesterday.clone().map(|u| u.close).unwrap_or(0.0f32))
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator_data: &KeltnerChannelResult) -> Option<f32> {
        if indicator_data.lower_band >= stock_price_info.low && indicator_data.lower_band >= stock_price_info.close { // between lower_band & ema
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator_data: &KeltnerChannelResult) -> Option<f32> {
        if indicator_data.upper_band <= stock_price_info.high {
            Some(indicator_data.upper_band)
        } else {
            None
        }
    }
}