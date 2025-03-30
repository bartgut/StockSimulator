use std::marker::PhantomData;
use crate::stock_data_reader::stock_data_reader::StockPriceInfo;
use crate::strategy_simulator::InvestingStrategy;

pub struct ChainedInvestingStrategy<T1, T2, S1, S2>
where
    S1: InvestingStrategy<T1>,
    S2: InvestingStrategy<T2>,
{
    strategy1: S1,
    strategy2: S2,
    _phantom1: PhantomData<T1>,
    _phantom2: PhantomData<T2>
}

impl<T1, T2, S1, S2> ChainedInvestingStrategy<T1, T2, S1, S2>
where
    S1: InvestingStrategy<T1>,
    S2: InvestingStrategy<T2>
{
    pub fn new(strategy1: S1, strategy2: S2) -> Self {
        ChainedInvestingStrategy {
            strategy1,
            strategy2,
            _phantom1: PhantomData,
            _phantom2: PhantomData
        }
    }
}


impl <T1, T2, S1, S2> InvestingStrategy<(T1, T2)> for ChainedInvestingStrategy<T1, T2, S1, S2>
where
    S1: InvestingStrategy<T1>,
    S2: InvestingStrategy<T2>
{
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> (T1, T2) {
        (self.strategy1.calculation(stock_price_info, yesterday), self.strategy2.calculation(stock_price_info, yesterday))
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &(T1, T2)) -> Option<f32> {
        let signal1 = self.strategy1.buy_signal(stock_price_info, &indicator.0);
        let signal2 = self.strategy2.buy_signal(stock_price_info, &indicator.1);

        match (signal1, signal2) {
            (Some(price1), Some(price2)) => Some(f32::max(price1, price2)),
            _ => None
        }
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &(T1, T2)) -> Option<f32> {
        let signal1 = self.strategy1.sell_signal(stock_price_info, &indicator.0);
        let signal2 = self.strategy2.sell_signal(stock_price_info, &indicator.1);

        match (signal1, signal2) {
            (Some(price1), Some(price2)) => Some(f32::min(price1, price2)),
            _ => None
        }
    }
}