// This method for calculating variance is adapted from the python
// implementation of welford's online algorithm on wikipedia at
// https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Parallel_algorithm
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

pub fn mean_and_variance(data: &[f64]) -> Option<(f64, f64, f64)> {
    let mut exag = ExistingAggregate::new();
    for x in data {
        exag.update(*x);
    }
    exag.finalize()
}

// Calculates the rolling mean for an array of f64.
// Because it is calculating over a range of window_size elements, the first
// window_size number of elements will not be calculated, and will be None.
pub fn rolling_mean(data: &[f64], window_size: usize) -> Vec<Option<f64>> {
    if window_size < 2 {
        panic!("window_size cannot be smaller than 2 in function rolling_mean");
    }
    if window_size > data.len() {
        panic!("window_size cannot be larger than data.len() in function rolling_mean");
    }

    let mut means_vec: Vec<Option<f64>> = vec![None; window_size - 1];
    let mut sum = data
        .iter()
        .zip(0..window_size)
        .fold(0.0_f64, |acc, (x, _)| acc + x);
    let window_size_f64 = window_size as f64;
    means_vec.push(Some(sum / window_size_f64));

    for i in window_size..data.len() {
        sum = sum - data[i - window_size] + data[i];
        means_vec.push(Some(sum / window_size_f64));
    }
    means_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_simple_list() {
        let mut exag = ExistingAggregate::new();
        for x in 1..=9 {
            // potential problem at upper bounds during conversion from i32 to f64
            exag.update(x as f64);
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

    #[test]
    fn simple_rolling_mean() {
        let data: Vec<f64> = (10..=70).step_by(10).map(|n| n as f64).collect();
        let window_size = 3;
        let means_vec = rolling_mean(&data, window_size);
        assert_eq!(
            vec![
                None,
                None,
                Some(20.0),
                Some(30.0),
                Some(40.0),
                Some(50.0),
                Some(60.0)
            ],
            means_vec
        );
    }
}
