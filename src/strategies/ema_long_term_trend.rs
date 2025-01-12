use crate::stock_data_reader::stock_data_reader::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::ema::Ema;

pub struct EmaLongTermTrendStrategy {
    ema: Ema,
    buy_percentage_diff_from_ema: f32,
    sell_percentage_diff_from_ema: f32
}

pub struct EmaLongTermTrendResult {
    ema: f32
}

impl EmaLongTermTrendStrategy {
    pub fn new(ema_length: usize,
               buy_percentage_diff_from_ema: f32,
               sell_percentage_diff_from_ema: f32) -> Self {
        Self {
            ema: Ema::new(ema_length),
            buy_percentage_diff_from_ema,
            sell_percentage_diff_from_ema
        }
    }
}

impl InvestingStrategy<EmaLongTermTrendResult> for EmaLongTermTrendStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, _: &Option<StockPriceInfo>) -> EmaLongTermTrendResult {
        let ema = self.ema.next(stock_price_info.close);
        EmaLongTermTrendResult {
            ema
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaLongTermTrendResult) -> Option<f32> {
        let percentage_change = (stock_price_info.close - indicator.ema)/indicator.ema;
        if percentage_change > self.buy_percentage_diff_from_ema {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaLongTermTrendResult) -> Option<f32> {
        let percentage_change = (indicator.ema - stock_price_info.close)/stock_price_info.close;
        if percentage_change > self.sell_percentage_diff_from_ema {
            Some(stock_price_info.close)
        } else {
            None
        }
    }
}