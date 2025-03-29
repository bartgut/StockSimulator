use crate::stock_data_reader::stock_data_reader::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::ema::Ema;
use crate::technical_indicator::keltner_channel::KeltnerChannelResult;

pub struct EmaCrossoverStrategy {
    ema_short: Ema,
    ema_long: Ema
}

#[derive(Clone)]
pub struct EmaCrossoverResult {
    ema_short: f32,
    ema_long: f32
}

impl EmaCrossoverStrategy {
    pub fn new(ema_short_length: usize, ema_long_length: usize) -> Self {
        Self {
            ema_short: Ema::new(ema_short_length),
            ema_long: Ema::new(ema_long_length)
        }
    }
}

impl Into<Vec<f32>> for EmaCrossoverResult {
    fn into(self) -> Vec<f32> {
        vec![self.ema_short, self.ema_long]
    }
}

impl InvestingStrategy<EmaCrossoverResult> for EmaCrossoverStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> EmaCrossoverResult {
        let new_ema_short = self.ema_short.next(stock_price_info.close);
        let new_ema_long = self.ema_long.next(stock_price_info.close);

        EmaCrossoverResult {
            ema_short: new_ema_short,
            ema_long: new_ema_long
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaCrossoverResult) -> Option<f32> {
        if indicator.ema_short > indicator.ema_long {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaCrossoverResult) -> Option<f32> {
        if indicator.ema_short < indicator.ema_long {
            Some(stock_price_info.close)
        } else {
            None
        }
    }
}
