use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::ema::Ema;

#[derive(Clone)]
pub struct EmaStrategyResult {
    yesterday_buy: f32,
    yesterday_sell: f32,
    today_buy: f32,
    today_sell: f32,
}

pub struct GrowingEmaStrategy {
    buy_ema: Ema,
    sell_ema: Ema,
    buy_inclination: f32,
    sell_inclination: f32
}

impl GrowingEmaStrategy {
    pub fn new(ema_length: usize, buy_inclination: f32, sell_inclination: f32) -> Self {
        Self {
            buy_ema: Ema::new(ema_length),
            sell_ema: Ema::new(ema_length),
            buy_inclination,
            sell_inclination
        }
    }

    pub fn with_separate_buy_sell_ema(buy_ema_length: usize,
                                      sell_ema_length: usize,
                                      buy_inclination: f32,
                                      sell_inclination: f32) -> Self {
        Self {
            buy_ema: Ema::new(buy_ema_length),
            sell_ema: Ema::new(sell_ema_length),
            buy_inclination,
            sell_inclination
        }
    }
}

impl InvestingStrategy<EmaStrategyResult> for GrowingEmaStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, _: &Option<StockPriceInfo>) -> EmaStrategyResult {
        EmaStrategyResult {
            yesterday_buy: self.buy_ema.current(),
            yesterday_sell: self.sell_ema.current(),
            today_buy: self.buy_ema.next(stock_price_info.close),
            today_sell: self.sell_ema.next(stock_price_info.close),
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaStrategyResult) -> Option<f32> {
        if calculate_inclination(indicator.yesterday_buy, indicator.today_buy) > self.buy_inclination {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaStrategyResult) -> Option<f32> {
        if calculate_inclination(indicator.yesterday_sell, indicator.today_sell) < self.sell_inclination {
            Some(stock_price_info.close)
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
