use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_analysis::keltner_channel::{KeltnerChannel, KeltnerChannelResult};

pub struct KeltnerChannelStrategyResult {
    yesterday: KeltnerChannelResult,
    today: KeltnerChannelResult
}

impl InvestingStrategy<KeltnerChannelStrategyResult> for KeltnerChannel {
    fn calculation(&mut self, today: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> KeltnerChannelStrategyResult {
        KeltnerChannelStrategyResult {
            yesterday: self.current(),
            today: self.next(today.close, today.high, today.low, yesterday.clone().map(|u| u.close).unwrap_or(0.0f32))
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator_data: &KeltnerChannelStrategyResult) -> Option<f32> {
        if indicator_data.today.lower_band >= stock_price_info.low && indicator_data.today.lower_band >= stock_price_info.close {
            //if calculate_inclination(indicator_data.yesterday.ema, indicator_data.today.ema) > 0.0 {
                Some(stock_price_info.close)
            //} else {
            //    None
            //}
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator_data: &KeltnerChannelStrategyResult) -> Option<f32> {
        if indicator_data.today.upper_band <= stock_price_info.high {
            Some(indicator_data.today.upper_band)
        } else {
            None
        }
    }
}

