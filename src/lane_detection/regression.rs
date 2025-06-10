pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn multiply_vectors(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    v1.iter().zip(v2.iter()).map(|(a, b)| a * b).collect()
}

pub fn linear_regression(x_values: &[f64], y_values: &[f64]) -> Option<(f64, f64)> {
    if x_values.is_empty() || y_values.is_empty() || x_values.len() != y_values.len() {
        return None;
    }

    let mean_x = mean(x_values);
    let mean_y = mean(y_values);

    let x_diffs: Vec<f64> = x_values.iter().map(|&x| x - mean_x).collect();
    let y_diffs: Vec<f64> = y_values.iter().map(|&y| y - mean_y).collect();

    let x_diffs_squared: Vec<f64> = x_diffs.iter().map(|&x_diff| x_diff * x_diff).collect();
    let xy_diffs_product = multiply_vectors(&x_diffs, &y_diffs);

    let sum_xy_diffs = xy_diffs_product.iter().sum::<f64>();
    let sum_x_diffs_squared = x_diffs_squared.iter().sum::<f64>();

    if sum_x_diffs_squared == 0.0 {
        return None;
    }

    let slope = sum_xy_diffs / sum_x_diffs_squared;
    let intercept = mean_y - slope * mean_x;

    Some((slope, intercept))
}