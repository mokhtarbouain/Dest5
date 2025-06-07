

use std::fmt;

pub fn print_estimated_state(state: &Vec<f64>) -> Result<(), std::fmt::Error> {
    if state.is_empty() {
        return Err(std::fmt::Error);
    }
    println!("Estimated state:");
    for (i, value) in state.iter().enumerate() {
        println!("x{}: {}", i + 1, value)?;
    }
    Ok(())
}

pub fn print_covariance_matrix(matrix: &Vec<Vec<f64>>) -> Result<(), std::fmt::Error> {
    if matrix.is_empty() || matrix.iter().any(|row| row.is_empty()) {
        return Err(std::fmt::Error);
    }
    println!("Covariance matrix:");
    for row in matrix {
        for value in row {
            print!("{} ", value)?;
        }
        println()?;
    }
    Ok(())
}