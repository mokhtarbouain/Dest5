

use num_traits::{Float, Num, NumCast, Zero};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub data: [[f64; 2]; 2],
}

impl Matrix {
    pub fn new(data: [[f64; 2]; 2]) -> Self {
        Matrix { data }
    }

    pub fn identity() -> Self {
        Matrix {
            data: [[1.0, 0.0], [0.0, 1.0]],
        }
    }

    pub fn mul(&self, other: &Matrix) -> Self {
        let mut result = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix { data: result }
    }

    pub fn add(&self, other: &Matrix) -> Self {
        let mut result = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Matrix { data: result }
    }
}