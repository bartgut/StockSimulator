use std::cmp::max;
use crate::broker_fee::BrokerFee;
use crate::keltner_channel::{KeltnerChannel, KeltnerChannelResult};
use crate::StockPriceInfo;
use crate::stop_loss_strategy::StopLoss;

pub trait InvestingStrategy<T> {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> T;
    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &T) -> Option<f32>;
    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &T) -> Option<f32>;
}

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

pub struct StrategySimulator<T> {
    strategy: Box<dyn InvestingStrategy<T>>,
    stop_loss: Box<dyn StopLoss>,
    broker_fee: Box<dyn BrokerFee>,
    cash: f32,
    last_buy_price: f32,
    current_position: usize,
}

impl<T> StrategySimulator<T>  {
    pub fn new(invested_cash: f32, strategy: Box<dyn InvestingStrategy<T>>, stop_loss: Box<dyn StopLoss>, broker_fee: Box<dyn BrokerFee>) -> Self<> {
        Self {
            strategy,
            stop_loss,
            broker_fee,
            cash: invested_cash,
            last_buy_price: 0.0f32,
            current_position: 0,
        }
    }

    pub fn next(&mut self, today: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) {
        let res = self.strategy.calculation(&today, yesterday);
        if self.current_position > 0 {
            if let Some(sell_price) = self.strategy.sell_signal(&today, &res) {
                self.sell_operation(sell_price);
                println!("{}: Selling at {}, cash: {}", today.date, sell_price, self.cash)
            }
            if let Some(stop_loss_price) = self.stop_loss.should_trigger_stop_loss(today, self.last_buy_price) {
                self.sell_operation(stop_loss_price);
                println!("{}: Stop loss triggered at {}, cash: {}", today.date, stop_loss_price, self.cash)
            }
        }
        if self.current_position == 0 {
            if let Some(buy_price) = self.strategy.buy_signal(&today, &res) {
                self.buy_operation(buy_price);
                println!("{}: Buying at {} number of shares: {}, cash left: {}", today.date, buy_price, self.current_position, self.cash)
            }
        }
    }

    fn sell_operation(&mut self, sell_price: f32) {
        self.cash = self.cash +
            sell_price * self.current_position as f32 -
            self.broker_fee.sell_fee(self.current_position, sell_price);
        self.current_position = 0;
    }

    fn buy_operation(&mut self, buy_price: f32) {
        let mut volume = (self.cash / buy_price) as usize;
        let mut operation_price = volume as f32 * buy_price;
        let mut operation_fee = self.broker_fee.buy_fee(volume, buy_price);
        let mut operation_price_with_fee = operation_price + operation_fee;
        while self.cash < operation_price_with_fee  {
            volume = volume - 1;
            operation_price = volume as f32 * buy_price;
            operation_fee = self.broker_fee.buy_fee(volume, buy_price);
            operation_price_with_fee = operation_price + operation_fee;
        }
        self.current_position = volume;
        self.cash = self.cash - operation_price_with_fee;
        self.last_buy_price = buy_price;
    }
}