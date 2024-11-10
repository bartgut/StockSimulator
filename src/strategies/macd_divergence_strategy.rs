use std::cmp::min;
use std::collections::VecDeque;

use crate::StockPriceInfo;
use crate::strategies::macd_strategy::MACDStrategy;
use crate::strategy_simulator::InvestingStrategy;
use crate::technical_indicator::macd::{Macd, MACDResult};
use crate::utils::rolling_window::RollingWindow;

pub struct MACDDivergence {
    macd: Macd,
    last_three_price: RollingWindow<f32>,
    result: MACDDivergenceResult
}

#[derive(Clone)]
pub struct MACDDivergenceResult {
    pub local_minimas: VecDeque<f32>,
    pub local_minima_macd: VecDeque<MACDResult>,
    pub current_macd_result: MACDResult
}

impl MACDDivergence {
    
    fn new() -> Self {
        MACDDivergence {
            result: MACDDivergenceResult {
                local_minimas: VecDeque::new(),
                local_minima_macd: VecDeque::new(),
                current_macd_result: MACDResult {
                    macd_line: 0.0,
                    signal_line: 0.0,
                }
            },
            macd: Macd::default(),
            last_three_price: RollingWindow::new(3)

        }
    }

    pub fn next(&mut self, price: f32) -> MACDDivergenceResult {
        let macd_result = self.macd.next(price);
        self.last_three_price.add(price);
        if self.is_local_minima() {
            self.result.local_minimas.push_back(*self.last_three_price.get(1).unwrap());
            self.result.local_minima_macd.push_back(macd_result);
            self.result.current_macd_result = macd_result;
        }
        self.result.clone()
    }

    fn is_local_minima(&self) -> bool {
        if let (Some(&first), Some(&middle), Some(&last)) = (
            self.last_three_price.get(0),
            self.last_three_price.get(1),
            self.last_three_price.get(2),
        ) {
            first > middle && middle < last
        } else {
            false
        }
    }
}


pub struct MACDDivergenceStrategy {
    macd_divergence: MACDDivergence 
}

impl MACDDivergenceStrategy {
    pub fn default() -> Self {
        MACDDivergenceStrategy {
            macd_divergence: MACDDivergence::new()
        }
    }
}

impl InvestingStrategy<MACDDivergenceResult> for MACDDivergenceStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> MACDDivergenceResult {
        self.macd_divergence.next(stock_price_info.close)
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &MACDDivergenceResult) -> Option<f32> {
        if indicator.local_minimas.len() > 2 {
            if let (Some(&last_minima), Some(&minima), Some(&last_macd_res), Some(&macd_res)) = (
                indicator.local_minimas.get(indicator.local_minimas.len() - 1),
                indicator.local_minimas.get(indicator.local_minimas.len() - 2),
                indicator.local_minima_macd.get(indicator.local_minima_macd.len() - 1).clone(),
                indicator.local_minima_macd.get(indicator.local_minima_macd.len() - 2).clone()
            ) {
                if minima - last_minima > 2.0 && last_macd_res.macd_line > macd_res.macd_line {
                    return Some(stock_price_info.close)
                } else {
                    return None
                }
            } else {
                return None
            }
        } else {
            return None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &MACDDivergenceResult) -> Option<f32> {
        if indicator.current_macd_result.macd_line > indicator.current_macd_result.signal_line {
            Some(stock_price_info.close)
        } else {
            None
        }
    }
}