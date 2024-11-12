pub struct Parameter {
    min: f32,
    max: f32,
    step: f32,
}

impl Parameter {
    pub fn new(min: f32, max: f32, step: f32) -> Self {
        Parameter { min, max, step }
    }

    pub fn values(&self) -> Vec<f32> {
        let mut values = Vec::new();
        let mut current = self.min;
        while current <= self.max {
            values.push(current);
            current += self.step
        }
        values
    }


}