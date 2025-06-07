

mod identity_matrix {
    use std::default::Default;

    #[derive(Debug, PartialEq)]
    pub struct Matrix {
        pub data: Vec<Vec<f64>>,
    }

    impl Default for Matrix {
        fn default() -> Self {
            Matrix { data: Vec::new() }
        }
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Self {
            let mut data = Vec::with_capacity(rows);
            for _ in 0..rows {
                let row = vec![0.0; cols];
                data.push(row);
            }
            Matrix { data }
        }

        pub fn identity(size: usize) -> Self {
            let mut matrix = Matrix::new(size, size);
            for i in 0..size {
                matrix.data[i][i] = 1.0;
            }
            matrix
        }
    }

    pub fn identity_matrix(size: usize) -> Matrix {
        Matrix::identity(size)
    }
}