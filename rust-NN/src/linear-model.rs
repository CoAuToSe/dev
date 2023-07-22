struct LinearModel {
    slope: f64,
    intercept: f64,
}
impl LinearModel {
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}
pub fn loss(&self, xs: &[f64], ys: &[f64]) -> f64 {
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| {
            let prediction = self.predict(*x);
            let error = y - prediction;
            error * error
        })
        .sum()
}
pub fn train(&mut self, xs: &[f64], ys: &[f64], learning_rate: f64, num_epochs: usize) {
    for _ in 0..num_epochs {
        let slope_gradient: f64 = xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| -2.0 * x * (y - self.predict(*x)))
            .sum();

        let intercept_gradient: f64 = xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| -2.0 * (y - self.predict(*x)))
            .sum();

        self.slope -= learning_rate * slope_gradient;
        self.intercept -= learning_rate * intercept_gradient;
    }
}
