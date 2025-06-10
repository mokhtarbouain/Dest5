use nalgebra::{Matrix2, Vector2};
use std::fmt;

#[derive(Debug)]
pub enum RegressionError {
    EmptyData,
    InsufficientData,
    ZeroVariance,
    DimensionMismatch,
}

impl fmt::Display for RegressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegressionError::EmptyData => write!(f, "Input data is empty"),
            RegressionError::InsufficientData => write!(f, "Insufficient data points for regression"),
            RegressionError::ZeroVariance => write!(f, "Zero variance in x values"),
            RegressionError::DimensionMismatch => write!(f, "X and Y data dimensions do not match"),
        }
    }
}

impl std::error::Error for RegressionError {}

pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

impl LinearRegression {
    pub fn new() -> Self {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    pub fn fit(&mut self, x: &[f64], y: &[f64]) -> Result<(), RegressionError> {
        let (slope, intercept) = fit_line(x, y)?;
        self.slope = slope;
        self.intercept = intercept;
        Ok(())
    }

    pub fn predict(&self, x: &[f64]) -> Vec<f64> {
        x.iter()
            .map(|&x_val| self.slope * x_val + self.intercept)
            .collect()
    }

    pub fn predict_single(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

impl Default for LinearRegression {
    fn default() -> Self {
        Self::new()
    }
}

pub fn fit_line(x: &[f64], y: &[f64]) -> Result<(f64, f64), RegressionError> {
    validate_input(x, y)?;

    let x_mean = calculate_mean(x);
    let y_mean = calculate_mean(y);
    
    let x_variance = calculate_variance(x, x_mean);
    if x_variance == 0.0 {
        return Err(RegressionError::ZeroVariance);
    }
    
    let covariance = calculate_covariance(x, y, x_mean, y_mean);
    
    let slope = covariance / x_variance;
    let intercept = y_mean - slope * x_mean;
    
    Ok((slope, intercept))
}

fn validate_input(x: &[f64], y: &[f64]) -> Result<(), RegressionError> {
    if x.is_empty() || y.is_empty() {
        return Err(RegressionError::EmptyData);
    }
    
    if x.len() != y.len() {
        return Err(RegressionError::DimensionMismatch);
    }
    
    if x.len() < 2 {
        return Err(RegressionError::InsufficientData);
    }
    
    Ok(())
}

fn calculate_mean(values: &[f64]) -> f64 {
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

fn calculate_variance(values: &[f64], mean: f64) -> f64 {
    values.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64
}

fn calculate_covariance(x: &[f64], y: &[f64], x_mean: f64, y_mean: f64) -> f64 {
    let mut covariance = 0.0;
    for i in 0..x.len() {
        covariance += (x[i] - x_mean) * (y[i] - y_mean);
    }
    covariance / x.len() as f64
}

pub fn fit_line_matrix(x: &[f64], y: &[f64]) -> Result<(f64, f64), RegressionError> {
    validate_input(x, y)?;
    
    let n = x.len();
    let x_sum: f64 = x.iter().sum();
    let y_sum: f64 = y.iter().sum();
    let xy_sum: f64 = x.iter().zip(y.iter()).map(|(&xi, &yi)| xi * yi).sum();
    let x_squared_sum: f64 = x.iter().map(|&xi| xi * xi).sum();
    
    let a = Matrix2::new(
        n as f64, x_sum,
        x_sum, x_squared_sum
    );
    
    let b = Vector2::new(y_sum, xy_sum);
    
    match a.try_inverse() {
        Some(a_inv) => {
            let solution = a_inv * b;
            Ok((solution[1], solution[0]))
        },
        None => Err(RegressionError::ZeroVariance),
    }
}