

mod vector {
    use std::ops::{Add, Sub};
    use std::fmt;

    pub struct Vector {
        data: Vec<f64>,
    }

    impl Vector {
        pub fn new(size: usize) -> Result<Vector, String> {
            if size == 0 {
                Err("Size must be greater than 0".to_string())
            } else {
                Ok(Vector {
                    data: vec![0.0; size],
                })
            }
        }

        pub fn add(&self, other: &Vector) -> Result<Vector, String> {
            if self.data.len() != other.data.len() {
                Err("Vectors must be of the same size".to_string())
            } else {
                Ok(Vector {
                    data: self
                        .data
                        .iter()
                        .zip(other.data.iter())
                        .map(|(&a, &b)| a + b)
                        .collect(),
                })
            }
        }

        pub fn subtract(&self, other: &Vector) -> Result<Vector, String> {
            if self.data.len() != other.data.len() {
                Err("Vectors must be of the same size".to_string())
            } else {
                Ok(Vector {
                    data: self
                        .data
                        .iter()
                        .zip(other.data.iter())
                        .map(|(&a, &b)| a - b)
                        .collect(),
                })
            }
        }
    }

    impl Add for Vector {
        type Output = Result<Vector, String>;

        fn add(self, other: Vector) -> Result<Vector, String> {
            self.add(&other)
        }
    }

    impl Sub for Vector {
        type Output = Result<Vector, String>;

        fn sub(self, other: Vector) -> Result<Vector, String> {
            self.subtract(&other)
        }
    }

    impl fmt::Display for Vector {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[")?;
            for (i, value) in self.data.iter().enumerate() {
                write!(f, "{}", value)?;
                if i < self.data.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")
        }
    }
}