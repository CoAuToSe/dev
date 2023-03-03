#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LearningParameters {
    pub learning_rate: f64,
    pub momentum: f64,
    pub weight_decay: f64
}