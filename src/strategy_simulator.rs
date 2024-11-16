use std::cmp::max;
use chrono::NaiveDate;
use crate::broker_fee::BrokerFee;
use crate::StockPriceInfo;
use crate::stop_loss_strategy::StopLossTrigger;
use crate::strategy_simulator::TradeResult::{Buy, Sell, StopLoss};

pub trait InvestingStrategy<T> {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> T;
    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &T) -> Option<f32>;
    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &T) -> Option<f32>;
}

pub struct StrategySimulator<T> {
    strategy: Box<dyn InvestingStrategy<T>>,
    stop_loss: Box<dyn StopLossTrigger>,
    broker_fee: Box<dyn BrokerFee>,
    cash: f32,
    start_date: NaiveDate,
    last_buy_price: f32,
    current_position: usize,
}

pub struct Trade {
    pub operation_date: NaiveDate,
    pub price: f32,
    pub after_operation_cash: f32
}

pub enum TradeResult {
    Buy(Trade),
    Sell(Trade),
    StopLoss(Trade)
}

impl<T> StrategySimulator<T>  {
    pub fn new(invested_cash: f32,
               start_date: NaiveDate,
               strategy: Box<dyn InvestingStrategy<T>>,
               stop_loss: Box<dyn StopLossTrigger>,
               broker_fee: Box<dyn BrokerFee>) -> Self<> {
        Self {
            strategy,
            stop_loss,
            broker_fee,
            cash: invested_cash,
            start_date: start_date,
            last_buy_price: 0.0f32,
            current_position: 0,
        }
    }

    pub fn next_today(&mut self, today: &StockPriceInfo) -> Vec<TradeResult> {
        self.next(today, &None)
    }

    pub fn next(&mut self, today: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> Vec<TradeResult> {
        let metric_result = self.strategy.calculation(&today, yesterday);
        let mut operations_performed = vec![];
        if today.date >= self.start_date {
            if self.current_position > 0 {
                if let Some(sell_price) = self.strategy.sell_signal(&today, &metric_result) {
                    self.sell_operation(sell_price);
                    //println!("{}: Selling at {}, cash: {}", today.date, sell_price, self.cash);
                    operations_performed.push(Sell(Trade {
                        operation_date: today.date.clone(),
                        price: sell_price,
                        after_operation_cash: self.cash
                    }));
                }
                if let Some(stop_loss_price) = self.stop_loss.should_trigger_stop_loss(today, self.last_buy_price) {
                    self.sell_operation(stop_loss_price);
                    //println!("{}: Stop loss triggered at {}, cash: {}", today.date, stop_loss_price, self.cash);
                    operations_performed.push(StopLoss(Trade {
                        operation_date: today.date.clone(),
                        price: stop_loss_price,
                        after_operation_cash: self.cash,
                    }))
                }
            }
            if self.current_position == 0 {
                if let Some(buy_price) = self.strategy.buy_signal(&today, &metric_result) {
                    self.buy_operation(buy_price);
                    //println!("{}: Buying at {} number of shares: {}, cash left: {}", today.date, buy_price, self.current_position, self.cash);
                    operations_performed.push(Buy(Trade {
                        operation_date: today.date.clone(),
                        price: buy_price,
                        after_operation_cash: self.cash,
                    }))
                }
            }
        }
        operations_performed
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