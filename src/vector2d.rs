

mod vector2d {
    use std::ops::{Add, Sub, Mul};

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
}