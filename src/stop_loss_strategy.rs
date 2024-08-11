use crate::StockPriceInfo;

pub trait StopLossTrigger {
    fn should_trigger_stop_loss(&self, stock_price_info: &StockPriceInfo, last_buy_price: f32) -> Option<f32>;
}

pub struct PercentageStopLoss {
    stop_loss_percentage: f32
}

impl PercentageStopLoss {
    pub fn new(stop_loss_percentage: f32) -> Self {
        Self {
            stop_loss_percentage
        }
    }
}

pub struct NoStopLoss;

impl StopLossTrigger for NoStopLoss {
    fn should_trigger_stop_loss(&self, _: &StockPriceInfo, _: f32) -> Option<f32> {
        None
    }
}

impl StopLossTrigger for PercentageStopLoss {
    fn should_trigger_stop_loss(&self, stock_price_info: &StockPriceInfo, last_buy_price: f32) -> Option<f32> {
       if stock_price_info.low <= last_buy_price * (1.0 - self.stop_loss_percentage) {
           Some(stock_price_info.low)
       } else {
           None
       }
    }
}



