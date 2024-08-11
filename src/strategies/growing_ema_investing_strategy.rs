use crate::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_analysis::ema::Ema;

pub struct EmaStrategyResult {
    yesterday: f32,
    today: f32
}

impl InvestingStrategy<EmaStrategyResult> for Ema {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, _: &Option<StockPriceInfo>) -> EmaStrategyResult {
        EmaStrategyResult {
            yesterday: self.current(),
            today: self.next(stock_price_info.close)
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaStrategyResult) -> Option<f32> {
        if calculate_inclination(indicator.yesterday, indicator.today) > 10.0  {
            Some(stock_price_info.close)
        } else {
            None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &EmaStrategyResult) -> Option<f32> {
        if calculate_inclination(indicator.yesterday, indicator.today) < -0.0 {
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
