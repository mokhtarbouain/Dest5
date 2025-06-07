

mod vector2d {
    use std::ops::{Add, Sub, Mul, Div};
    use std::fmt;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Vector2D {
        pub x: f64,
        pub y: f64,
    }

    impl Vector2D {
        /// Creates a new `Vector2D` instance with the given `x` and `y` coordinates.
        pub fn new(x: f64, y: f64) -> Vector2D {
            Vector2D { x, y }
        }

        /// Adds the given `other` vector to the current vector.
        pub fn add(&self, other: &Vector2D) -> Vector2D {
            Vector2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }

        /// Subtracts the given `other` vector from the current vector.
        pub fn sub(&self, other: &Vector2D) -> Vector2D {
            Vector2D {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }

        /// Scales the current vector by the given `scalar` value.
        pub fn scale(&self, scalar: f64) -> Vector2D {
            Vector2D {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }

        /// Divides the current vector by the given `scalar` value.
        pub fn div(&self, scalar: f64) -> Result<Vector2D, &'static str> {
            if scalar == 0.0 {
                Err("Cannot divide by zero")
            } else {
                Ok(Vector2D {
                    x: self.x / scalar,
                    y: self.y / scalar,
                })
            }
        }

        /// Calculates the magnitude (length) of the current vector.
        pub fn magnitude(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        /// Calculates the dot product of the current vector and the given `other` vector.
        pub fn dot(&self, other: &Vector2D) -> f64 {
            self.x * other.x + self.y * other.y
        }

        /// Calculates the cross product of the current vector and the given `other` vector.
        pub fn cross(&self, other: &Vector2D) -> f64 {
            self.x * other.y - self.y * other.x
        }

        /// Normalizes the current vector to have a length of 1.
        pub fn normalize(&self) -> Vector2D {
            let magnitude = self.magnitude();
            Vector2D {
                x: self.x / magnitude,
                y: self.y / magnitude,
            }
        }
    }

    impl Add for Vector2D {
        type Output = Vector2D;

        fn add(self, other: Vector2D) -> Vector2D {
            Vector2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub for Vector2D {
        type Output = Vector2D;

        fn sub(self, other: Vector2D) -> Vector2D {
            Vector2D {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul<f64> for Vector2D {
        type Output = Vector2D;

        fn mul(self, scalar: f64) -> Vector2D {
            Vector2D {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    impl Div<f64> for Vector2D {
        type Output = Result<Vector2D, &'static str>;

        fn div(self, scalar: f64) -> Result<Vector2D, &'static str> {
            if scalar == 0.0 {
                Err("Cannot divide by zero")
            } else {
                Ok(Vector2D {
                    x: self.x / scalar,
                    y: self.y / scalar,
                })
            }
        }
    }

    impl fmt::Display for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}