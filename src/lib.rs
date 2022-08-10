#[derive(Default)]
pub struct ExistingAggregate {
    pub count: f64,
    pub mean: f64,
    pub m2: f64,
}

impl ExistingAggregate {
    pub fn new() -> ExistingAggregate {
        Default::default()
    }

    pub fn update(&mut self, new_value: f64) -> &mut ExistingAggregate {
        self.count += 1.0;
        let delta = new_value - self.mean;
        self.mean += delta / self.count;
        let delta2 = new_value - self.mean;
        self.m2 += delta * delta2;
        self
    }

    pub fn finalize(&self) -> Option<(f64, f64, f64)> {
        if self.count < 2.0 {
            None
        } else {
            let mean = self.mean;
            let variance = self.m2 / self.count;
            let sample_variance = self.m2 / (self.count - 1.0);
            Some((mean, variance, sample_variance))
        }
    }
}
