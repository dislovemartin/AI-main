use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyDetector {
    window_size: usize,
    threshold: f64,
    history: VecDeque<f64>,
    baseline_stats: BaselineStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct BaselineStats {
    mean: f64,
    std_dev: f64,
    min: f64,
    max: f64,
}

impl AnomalyDetector {
    pub fn new(window_size: usize, threshold: f64) -> Self {
        Self {
            window_size,
            threshold,
            history: VecDeque::with_capacity(window_size),
            baseline_stats: BaselineStats {
                mean: 0.0,
                std_dev: 0.0,
                min: f64::MAX,
                max: f64::MIN,
            },
        }
    }

    pub fn update_baseline(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let n = self.history.len() as f64;
        let sum: f64 = self.history.iter().sum();
        let mean = sum / n;

        let variance = self.history.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;

        self.baseline_stats = BaselineStats {
            mean,
            std_dev: variance.sqrt(),
            min: *self
                .history
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            max: *self
                .history
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        };
    }

    pub fn detect(&mut self, value: f64) -> bool {
        if self.history.len() >= self.window_size {
            self.history.pop_front();
        }
        self.history.push_back(value);
        self.update_baseline();

        if self.baseline_stats.std_dev == 0.0 {
            return false;
        }

        let z_score = (value - self.baseline_stats.mean) / self.baseline_stats.std_dev;
        z_score.abs() > self.threshold
    }

    pub fn get_anomaly_score(&self, value: f64) -> f64 {
        (value - self.baseline_stats.mean).abs() / self.baseline_stats.std_dev
    }

    pub fn detect_with_isolation_forest(&mut self, value: f64) -> bool {
        if self.history.len() >= self.window_size {
            self.history.pop_front();
        }
        self.history.push_back(value);

        let anomaly_score = self.calculate_isolation_score();
        anomaly_score > self.threshold
    }

    fn calculate_isolation_score(&self) -> f64 {
        let mut score = 0.0;
        let data: Vec<f64> = self.history.iter().copied().collect();

        for (i, point) in data.iter().enumerate() {
            let path_length = self.estimate_path_length(*point, &data, i);
            score += path_length;
        }

        let avg_path_length = score / data.len() as f64;
        let normalized_score =
            2.0_f64.powf(-avg_path_length / self.calculate_average_path_length(data.len()));
        normalized_score
    }

    fn estimate_path_length(&self, point: f64, data: &[f64], exclude_idx: usize) -> f64 {
        let mut path_length = 0.0;
        let mut current_range = (f64::MIN, f64::MAX);
        let mut remaining_points: Vec<(usize, f64)> = data
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != exclude_idx)
            .map(|(i, &v)| (i, v))
            .collect();

        while !remaining_points.is_empty() && path_length < 100.0 {
            let split_value = self.choose_split_value(&remaining_points, current_range);
            let (left, right): (Vec<_>, Vec<_>) = remaining_points
                .into_iter()
                .partition(|&(_, v)| v < split_value);

            if point < split_value {
                current_range.1 = split_value;
                remaining_points = left;
            } else {
                current_range.0 = split_value;
                remaining_points = right;
            }
            path_length += 1.0;
        }

        path_length
    }

    fn choose_split_value(&self, points: &[(usize, f64)], range: (f64, f64)) -> f64 {
        if points.is_empty() {
            return range.0;
        }

        let min_val = points.iter().map(|&(_, v)| v).fold(f64::INFINITY, f64::min);
        let max_val = points
            .iter()
            .map(|&(_, v)| v)
            .fold(f64::NEG_INFINITY, f64::max);

        (min_val + max_val) / 2.0
    }

    fn calculate_average_path_length(&self, size: usize) -> f64 {
        if size <= 1 {
            return 0.0;
        }

        let size_f = size as f64;
        2.0 * (size_f.ln() + 0.5772156649) - (2.0 * (size_f - 1.0) / size_f)
    }

    pub fn detect_with_robust_statistics(&mut self, value: f64) -> bool {
        if self.history.len() >= self.window_size {
            self.history.pop_front();
        }
        self.history.push_back(value);
        self.update_baseline();

        let mad = self.calculate_median_absolute_deviation();
        let median = self.calculate_median();

        let deviation = (value - median).abs();
        (deviation / mad) > self.threshold
    }

    fn calculate_median_absolute_deviation(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }

        let median = self.calculate_median();
        let deviations: Vec<f64> = self.history.iter().map(|&x| (x - median).abs()).collect();

        let mut sorted_deviations = deviations.clone();
        sorted_deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted_deviations.len() / 2;
        if sorted_deviations.len() % 2 == 0 {
            (sorted_deviations[mid - 1] + sorted_deviations[mid]) / 2.0
        } else {
            sorted_deviations[mid]
        }
    }

    fn calculate_median(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let mut sorted = self.history.iter().copied().collect::<Vec<f64>>();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }
}
