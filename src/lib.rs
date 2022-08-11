// This method for calculating variance is adapted from the python
// implementation of welford's online algorithm on wikipedia at
// https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Parallel_algorithm
#[derive(Default)]
pub struct ExistingAggregate {
    pub count: f32,
    pub mean: f32,
    pub m2: f32,
}

impl ExistingAggregate {
    pub fn new() -> ExistingAggregate {
        Default::default()
    }

    pub fn update(&mut self, new_value: f32) -> &mut ExistingAggregate {
        self.count += 1.0;
        let delta = new_value - self.mean;
        self.mean += delta / self.count;
        let delta2 = new_value - self.mean;
        self.m2 += delta * delta2;
        self
    }

    pub fn finalize(&self) -> Option<(f32, f32, f32)> {
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

pub fn mean_and_variance(data: &[f32]) -> Option<(f32, f32, f32)> {
    let mut exag = ExistingAggregate::new();
    for x in data {
        exag.update(*x);
    }
    exag.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_simple_list() {
        let mut exag = ExistingAggregate::new();
        for x in 1..=9 {
            // potential problem at upper bounds during conversion from i32 to f32
            exag.update(x as f32);
        }
        let (mean, variance, sample_variance) = exag.finalize().unwrap();
        assert_eq!(5.0, mean);
        assert_eq!(6.667, (variance * 1000.0).round() / 1000.0);
        assert_eq!(7.5, sample_variance);
    }

    #[test]
    fn empty_sample() {
        let exag = ExistingAggregate::new();
        assert_eq!(None, exag.finalize());
    }

    #[test]
    fn one_sample() {
        let mut exag = ExistingAggregate::new();
        exag.update(1.0);
        assert_eq!(None, exag.finalize());
    }

    #[test]
    fn three_ones() {
        let mut exag = ExistingAggregate::new();
        for _ in 0..3 {
            exag.update(1.0);
        }
        let (mean, variance, sample_variance) = exag.finalize().unwrap();
        assert_eq!(1.0, mean);
        assert_eq!(0.0, variance);
        assert_eq!(0.0, sample_variance);
    }
}
