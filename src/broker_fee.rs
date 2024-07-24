pub trait BrokerFee {
    fn buy_fee(&self, shares: usize, price_per_share: f32) -> f32;
    fn sell_fee(&self, shares: usize, price_per_share: f32) -> f32;
}

pub struct PricePercentageFee {
    percentage: f32
}

impl PricePercentageFee {
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage
        }
    }
}

impl BrokerFee for PricePercentageFee {
    fn buy_fee(&self, shares: usize, price_per_share: f32) -> f32 {
        price_per_share * shares as f32 * self.percentage
    }

    fn sell_fee(&self, shares: usize, price_per_share: f32) -> f32 {
        price_per_share * shares as f32 * self.percentage
    }
}

