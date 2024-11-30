/// Module for evaluation metrics and related functionalities.
pub mod evaluation {
    /// Calculates the accuracy of predictions.
    ///
    /// # Examples
    ///
    /// ```
    /// let predictions = vec![1, 0, 1, 1];
    /// let targets = vec![1, 0, 0, 1];
    /// let acc = evaluation::accuracy(&predictions, &targets);
    /// assert_eq!(acc, 0.75);
    /// ```
    pub fn accuracy(predictions: &[i32], targets: &[i32]) -> f64 {
        let correct = predictions.iter().zip(targets.iter()).filter(|(p, t)| p == t).count();
        correct as f64 / targets.len() as f64
    }

    /// Calculates the precision of predictions.
    pub fn precision(predictions: &[i32], targets: &[i32]) -> f64 {
        let true_positive =
            predictions.iter().zip(targets.iter()).filter(|(p, t)| *p == 1 && *t == 1).count();
        let predicted_positive = predictions.iter().filter(|&&p| p == 1).count();
        if predicted_positive == 0 {
            0.0
        } else {
            true_positive as f64 / predicted_positive as f64
        }
    }

    /// Calculates the recall of predictions.
    pub fn recall(predictions: &[i32], targets: &[i32]) -> f64 {
        let true_positive =
            predictions.iter().zip(targets.iter()).filter(|(p, t)| *p == 1 && *t == 1).count();
        let actual_positive = targets.iter().filter(|&&t| t == 1).count();
        if actual_positive == 0 {
            0.0
        } else {
            true_positive as f64 / actual_positive as f64
        }
    }

    /// Calculates the F1 score of predictions.
    pub fn f1_score(predictions: &[i32], targets: &[i32]) -> f64 {
        let prec = precision(predictions, targets);
        let rec = recall(predictions, targets);
        if prec + rec == 0.0 {
            0.0
        } else {
            2.0 * prec * rec / (prec + rec)
        }
    }

    // Add more evaluation metrics as needed.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy() {
        let predictions = vec![1, 0, 1, 1];
        let targets = vec![1, 0, 0, 1];
        assert_eq!(accuracy(&predictions, &targets), 0.75);
    }

    #[test]
    fn test_precision() {
        let predictions = vec![1, 0, 1, 1];
        let targets = vec![1, 0, 0, 1];
        assert_eq!(precision(&predictions, &targets), 0.6666666666666666);
    }

    #[test]
    fn test_recall() {
        let predictions = vec![1, 0, 1, 1];
        let targets = vec![1, 0, 0, 1];
        assert_eq!(recall(&predictions, &targets), 1.0);
    }

    #[test]
    fn test_f1_score() {
        let predictions = vec![1, 0, 1, 1];
        let targets = vec![1, 0, 0, 1];
        assert_eq!(f1_score(&predictions, &targets), 0.8);
    }
}
