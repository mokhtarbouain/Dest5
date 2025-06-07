

mod matrix {
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub struct Matrix {
        data: Vec<Vec<f64>>,
        rows: usize,
        cols: usize,
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            let mut data = Vec::with_capacity(rows);
            for _ in 0..rows {
                let row = vec![0.0; cols];
                data.push(row);
            }
            Matrix { data, rows, cols }
        }

        pub fn identity(size: usize) -> Matrix {
            let mut data = Vec::with_capacity(size);
            for i in 0..size {
                let mut row = vec![0.0; size];
                row[i] = 1.0;
                data.push(row);
            }
            Matrix { data, rows: size, cols: size }
        }

        pub fn multiply(&self, other: &Matrix) -> Matrix {
            if self.cols != other.rows {
                panic!("Matrix dimensions are incompatible for multiplication");
            }
            let mut result = Matrix::new(self.rows, other.cols);
            for i in 0..self.rows {
                for j in 0..other.cols {
                    for k in 0..self.cols {
                        result.data[i][j] += self.data[i][k] * other.data[k][j];
                    }
                }
            }
            result
        }
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for row in &self.data {
                for val in row {
                    write!(f, "{} ", val)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }
}