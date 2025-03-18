

mod vector2d {
    use std::f64;

    /// A 2D vector with x and y coordinates.
    #[derive(Debug, PartialEq)]
    pub struct Vector2D {
        /// The x-coordinate of the vector.
        pub x: f64,
        /// The y-coordinate of the vector.
        pub y: f64,
    }

    impl Vector2D {
        /// Creates a new 2D vector with the given x and y coordinates.
        ///
        /// # Arguments
        ///
        /// * `x` - The x-coordinate of the vector.
        /// * `y` - The y-coordinate of the vector.
        ///
        /// # Returns
        ///
        /// A new `Vector2D` instance.
        pub fn new(x: f64, y: f64) -> Vector2D {
            Vector2D { x, y }
        }

        /// Adds two 2D vectors element-wise.
        ///
        /// # Arguments
        ///
        /// * `other` - The vector to add to the current vector.
        ///
        /// # Returns
        ///
        /// A new `Vector2D` instance representing the sum of the two vectors.
        pub fn add(&self, other: &Vector2D) -> Vector2D {
            Vector2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }

        /// Subtracts one 2D vector from another element-wise.
        ///
        /// # Arguments
        ///
        /// * `other` - The vector to subtract from the current vector.
        ///
        /// # Returns
        ///
        /// A new `Vector2D` instance representing the difference of the two vectors.
        pub fn subtract(&self, other: &Vector2D) -> Vector2D {
            Vector2D {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }

        /// Scales a 2D vector by a scalar value.
        ///
        /// # Arguments
        ///
        /// * `scalar` - The scalar value to scale the vector by.
        ///
        /// # Returns
        ///
        /// A new `Vector2D` instance representing the scaled vector.
        pub fn scale(&self, scalar: f64) -> Vector2D {
            if scalar == 0.0 {
                panic!("Cannot scale by zero");
            }
            Vector2D {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }

        /// Calculates the magnitude or length of the vector.
        ///
        /// # Returns
        ///
        /// The magnitude of the vector.
        pub fn magnitude(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let vector = Vector2D::new(1.0, 2.0);
            assert_eq!(vector.x, 1.0);
            assert_eq!(vector.y, 2.0);
        }

        #[test]
        fn test_add() {
            let vector1 = Vector2D::new(1.0, 2.0);
            let vector2 = Vector2D::new(3.0, 4.0);
            let result = vector1.add(&vector2);
            assert_eq!(result.x, 4.0);
            assert_eq!(result.y, 6.0);
        }

        #[test]
        fn test_subtract() {
            let vector1 = Vector2D::new(1.0, 2.0);
            let vector2 = Vector2D::new(3.0, 4.0);
            let result = vector1.subtract(&vector2);
            assert_eq!(result.x, -2.0);
            assert_eq!(result.y, -2.0);
        }

        #[test]
        fn test_scale() {
            let vector = Vector2D::new(1.0, 2.0);
            let result = vector.scale(2.0);
            assert_eq!(result.x, 2.0);
            assert_eq!(result.y, 4.0);
        }

        #[test]
        #[should_panic]
        fn test_scale_by_zero() {
            let vector = Vector2D::new(1.0, 2.0);
            vector.scale(0.0);
        }

        #[test]
        fn test_magnitude() {
            let vector = Vector2D::new(3.0, 4.0);
            assert_eq!(vector.magnitude(), 5.0);
        }
    }
}