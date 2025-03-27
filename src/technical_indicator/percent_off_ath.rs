pub struct PercentOffAth {
    ath: f32,
    current_percent_off_ath: f32
}

impl PercentOffAth {
    pub fn new() -> Self {
        Self {
            ath: 0.0f32,
            current_percent_off_ath: 0.0f32
        }
    }

    pub fn next(&mut self, price: f32) -> f32 {
        self.ath = f32::max(self.ath, price);
        self.current_percent_off_ath = ((self.ath - price) / self.ath) * 100.0;
        self.current_percent_off_ath
    }

}