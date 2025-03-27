use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::keltner_channel::{KeltnerChannel, KeltnerChannelResult};

#[derive(Clone)]
pub struct KeltnerChannelStrategyResult {
    pub yesterday: KeltnerChannelResult,
    pub today: KeltnerChannelResult
}

impl InvestingStrategy<KeltnerChannelStrategyResult> for KeltnerChannel {
    fn calculation(&mut self, today: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> KeltnerChannelStrategyResult {
        KeltnerChannelStrategyResult {
            yesterday: self.current(),
            today: self.next(today.close, today.high, today.low, yesterday.clone().map(|u| u.close).unwrap_or(0.0f32))
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator_data: &KeltnerChannelStrategyResult) -> Option<f32> {
        let keltner_buy = indicator_data.today.lower_band - stock_price_info.close;
        let signal = keltner_buy;
        if signal > 0.0 {
            Some(stock_price_info.close)
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

fn calculate_inclination(yesterday_ema: f32, today_ema: f32) -> f32 {
    let m = today_ema - yesterday_ema;
    let theta_radians = m.atan();
    theta_radians.to_degrees()
}
