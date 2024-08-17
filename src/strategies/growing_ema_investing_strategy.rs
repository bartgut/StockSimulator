use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::ema::Ema;

pub struct EmaStrategyResult {
    yesterday: f32,
    today: f32
}

pub struct GrowingEmaStrategy {
    ema: Ema,
    buy_inclination: f32,
    sell_inclination: f32
}

impl GrowingEmaStrategy {
    pub fn new(ema_length: usize, buy_inclination: f32, sell_inclination: f32) -> Self {
        Self {
            ema: Ema::new(ema_length),
            buy_inclination,
            sell_inclination
        }
    }
}

impl InvestingStrategy<EmaStrategyResult> for GrowingEmaStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, _: &Option<StockPriceInfo>) -> EmaStrategyResult {
        EmaStrategyResult {
            yesterday: self.ema.current(),
            today: self.ema.next(stock_price_info.close)
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaStrategyResult) -> Option<f32> {
        if calculate_inclination(indicator.yesterday, indicator.today) > self.buy_inclination  {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaStrategyResult) -> Option<f32> {
        if calculate_inclination(indicator.yesterday, indicator.today) < self.sell_inclination {
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
