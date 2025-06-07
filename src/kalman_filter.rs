

use std::f64;

#[derive(Debug, Clone, Copy)]
struct Matrix2x2 {
    data: [[f64; 2]; 2],
}

impl Matrix2x2 {
    fn new(data: [[f64; 2]; 2]) -> Self {
        Matrix2x2 { data }
    }

    fn identity() -> Self {
        Matrix2x2 {
            data: [[1.0, 0.0], [0.0, 1.0]],
        }
    }

    fn transpose(&self) -> Self {
        Matrix2x2 {
            data: [
                [self.data[0][0], self.data[1][0]],
                [self.data[0][1], self.data[1][1]],
            ],
        }
    }

    fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    fn inverse(&self) -> Self {
        let det = self.determinant();
        Matrix2x2 {
            data: [
                [self.data[1][1] / det, -self.data[0][1] / det],
                [-self.data[1][0] / det, self.data[0][0] / det],
            ],
        }
    }
}

impl std::ops::Add for Matrix2x2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Matrix2x2 {
            data: [
                [self.data[0][0] + other.data[0][0], self.data[0][1] + other.data[0][1]],
                [self.data[1][0] + other.data[1][0], self.data[1][1] + other.data[1][1]],
            ],
        }
    }
}

impl std::ops::Sub for Matrix2x2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Matrix2x2 {
            data: [
                [self.data[0][0] - other.data[0][0], self.data[0][1] - other.data[0][1]],
                [self.data[1][0] - other.data[1][0], self.data[1][1] - other.data[1][1]],
            ],
        }
    }
}

impl std::ops::Mul for Matrix2x2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Matrix2x2 {
            data: [
                [
                    self.data[0][0] * other.data[0][0] + self.data[0][1] * other.data[1][0],
                    self.data[0][0] * other.data[0][1] + self.data[0][1] * other.data[1][1],
                ],
                [
                    self.data[1][0] * other.data[0][0] + self.data[1][1] * other.data[1][0],
                    self.data[1][0] * other.data[0][1] + self.data[1][1] * other.data[1][1],
                ],
            ],
        }
    }
}

impl std::ops::Mul<f64> for Matrix2x2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Matrix2x2 {
            data: [
                [self.data[0][0] * scalar, self.data[0][1] * scalar],
                [self.data[1][0] * scalar, self.data[1][1] * scalar],
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    data: [f64; 2],
}

impl Vector2 {
    fn new(data: [f64; 2]) -> Self {
        Vector2 { data }
    }

    fn zeros() -> Self {
        Vector2 { data: [0.0, 0.0] }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2 {
            data: [self.data[0] + other.data[0], self.data[1] + other.data[1]],
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2 {
            data: [self.data[0] - other.data[0], self.data[1] - other.data[1]],
        }
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector2 {
            data: [self.data[0] * scalar, self.data[1] * scalar],
        }
    }
}

impl std::ops::Mul<Vector2> for Matrix2x2 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2 {
            data: [
                self.data[0][0] * other.data[0] + self.data[0][1] * other.data[1],
                self.data[1][0] * other.data[0] + self.data[1][1] * other.data[1],
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct KalmanFilter2D {
    state: Vector2,
    covariance: Matrix2x2,
    process_noise: Matrix2x2,
    measurement_noise: Matrix2x2,
    velocity: Vector2,
    error_covariance: Matrix2x2,
}

impl KalmanFilter2D {
    fn new(
        state: Vector2,
        covariance: Matrix2x2,
        process_noise: Matrix2x2,
        measurement_noise: Matrix2x2,
        velocity: Vector2,
        error_covariance: Matrix2x2,
    ) -> Self {
        assert!(
            !state.data[0].is_nan() && !state.data[1].is_nan(),
            "State cannot contain NaN values"
        );
        assert!(
            !velocity.data[0].is_nan() && !velocity.data[1].is_nan(),
            "Velocity cannot contain NaN values"
        );
        assert!(
            !process_noise.data[0][0].is_nan()
                && !process_noise.data[0][1].is_nan()
                && !process_noise.data[1][0].is_nan()
                && !process_noise.data[1][1].is_nan(),
            "Process noise cannot contain NaN values"
        );
        assert!(
            !measurement_noise.data[0][0].is_nan()
                && !measurement_noise.data[0][1].is_nan()
                && !measurement_noise.data[1][0].is_nan()
                && !measurement_noise.data[1][1].is_nan(),
            "Measurement noise cannot contain NaN values"
        );
        assert!(
            !covariance.data[0][0].is_nan()
                && !covariance.data[0][1].is_nan()
                && !covariance.data[1][0].is_nan()
                && !covariance.data[1][1].is_nan(),
            "Covariance cannot contain NaN values"
        );
        assert!(
            !error_covariance.data[0][0].is_nan()
                && !error_covariance.data[0][1].is_nan()
                && !error_covariance.data[1][0].is_nan()
                && !error_covariance.data[1][1].is_nan(),
            "Error covariance cannot contain NaN values"
        );
        assert_eq!(
            process_noise.data[0][0], process_noise.data[1][1],
            "Process noise must be a diagonal matrix"
        );
        assert_eq!(
            measurement_noise.data[0][0], measurement_noise.data[1][1],
            "Measurement noise must be a diagonal matrix"
        );
        assert!(
            covariance.data[0][0] >= 0.0 && covariance.data[1][1] >= 0.0,
            "Covariance must be a positive semi-definite matrix"
        );
        assert!(
            error_covariance.data[0][0] >= 0.0 && error_covariance.data[1][1] >= 0.0,
            "Error covariance must be a positive semi-definite matrix"
        );
        KalmanFilter2D {
            state,
            covariance,
            process_noise,
            measurement_noise,
            velocity,
            error_covariance,
        }
    }

    fn predict(&mut self) {
        self.state = self.state + self.velocity;
        self.covariance = self.covariance + self.process_noise;
        self.error_covariance = self.covariance;
    }

    fn update(&mut self, measurement: Vector2) {
        assert!(
            !measurement.data[0].is_nan() && !measurement.data[1].is_nan(),
            "Measurement cannot contain NaN values"
        );
        let innovation = measurement - self.state;
        let innovation_covariance = self.measurement_noise + self.error_covariance;
        let kalman_gain = self.error_covariance * innovation_covariance.inverse();
        self.state = self.state + kalman_gain * innovation;
        self.covariance = (Matrix2x2::identity() - kalman_gain) * self.error_covariance;
        self.error_covariance = self.covariance;
    }

    fn get_state(&self) -> Vector2 {
        self.state
    }
}