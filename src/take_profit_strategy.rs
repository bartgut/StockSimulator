use crate::stock_data_reader::stock_data_reader::StockPriceInfo;

pub trait TakeProfitTrigger {
    fn should_trigger_take_profit(&self, stock_price_info: &StockPriceInfo, last_buy_price: f32) -> Option<f32>;
}

pub struct PercentageTakeProfit {
    take_profit_percentage: f32
}

impl PercentageTakeProfit {
    pub fn new(take_profit_percentage: f32) -> Self {
        Self {
            take_profit_percentage
        }
    }
}

impl TakeProfitTrigger for PercentageTakeProfit {
    fn should_trigger_take_profit(&self, stock_price_info: &StockPriceInfo, last_buy_price: f32) -> Option<f32> {
        if stock_price_info.high >= last_buy_price * self.take_profit_percentage {
            Some(stock_price_info.high)
        } else {
            None
        }
    }
}

pub struct NoTakeProfit;

impl TakeProfitTrigger for NoTakeProfit {
    fn should_trigger_take_profit(&self, _: &StockPriceInfo, _: f32) -> Option<f32> {
        None
    }
}