

use super::matrix::Matrix;

#[test]
fn new_matrix() {
    let matrix = Matrix::new(2, 3);
    assert_eq!(matrix.rows, 2);
    assert_eq!(matrix.cols, 3);
}

#[test]
fn identity_matrix() {
    let matrix = Matrix::identity(3);
    assert_eq!(matrix.rows, 3);
    assert_eq!(matrix.cols, 3);
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                assert_eq!(matrix.data[i][j], 1);
            } else {
                assert_eq!(matrix.data[i][j], 0);
            }
        }
    }
}

#[test]
fn matrix_mul() {
    let matrix1 = Matrix::new(2, 3);
    let matrix2 = Matrix::new(3, 2);
    let result = matrix1.mul(&matrix2);
    assert_eq!(result.rows, 2);
    assert_eq!(result.cols, 2);
}

#[test]
fn matrix_add() {
    let matrix1 = Matrix::new(2, 3);
    let matrix2 = Matrix::new(2, 3);
    let result = matrix1.add(&matrix2);
    assert_eq!(result.rows, 2);
    assert_eq!(result.cols, 3);
}