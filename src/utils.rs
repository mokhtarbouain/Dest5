```rust
use ndarray::Array2;
use ndarray::Array1;
use ndarray::prelude::*;
use ndarray::linalg::Dot;

pub fn estimate_coefficients(x: &Array1<f64>, y: &Array1<f64>) -> (f64, f64) {
    let n = x.len() as f64;
    let x_mean = x.mean().unwrap();
    let y_mean = y.mean().unwrap();

    let ss_xy = x.iter().zip(y.iter()).map(|(&xi, &yi)| (xi - x_mean) * (yi - y_mean)).sum::<f64>();
    let ss_xx = x.iter().map(|&xi| (xi - x_mean).powi(2)).sum::<f64>();

    let slope = ss_xy / ss_xx;
    let intercept = y_mean - slope * x_mean;

    (slope, intercept)
}

pub fn predict(x: f64, slope: f64, intercept: f64) -> f64 {
    slope * x + intercept
}

pub fn linear_regression(x: &Array1<f64>, y: &Array1<f64>) -> Array1<f64> {
    let (slope, intercept) = estimate_coefficients(x, y);
    x.mapv(|xi| predict(xi, slope, intercept))
}
```