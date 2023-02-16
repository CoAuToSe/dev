use std::time::Duration;

#[derive(Clone, Debug)]
pub struct TrainingResult {
    pub duration: Duration,
    pub epochs: u64,
    pub error: f64,
    pub min_error: f64
}