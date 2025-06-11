use anyhow::{Result, anyhow};

/// Performs linear regression on x and y data points
/// Returns (slope, intercept) if regression is possible
pub fn linear_regression(x: &[f64], y: &[f64]) -> Result<Option<(f64, f64)>> {
    // Check if input vectors have the same length
    if x.len() != y.len() {
        return Err(anyhow!("Input vectors must have the same length"));
    }

    // Check if there are enough points for regression
    if x.len() < 2 {
        return Ok(None);
    }

    // Calculate means of x and y
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;

    // Calculate numerator and denominator for slope
    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for i in 0..x.len() {
        let x_diff = x[i] - mean_x;
        numerator += x_diff * (y[i] - mean_y);
        denominator += x_diff * x_diff;
    }

    // Check if denominator is close to zero to avoid division by zero
    if denominator.abs() < 1e-10 {
        return Ok(None);
    }

    // Calculate slope and intercept
    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;

    Ok(Some((slope, intercept)))
}