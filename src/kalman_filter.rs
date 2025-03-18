

use std::f64::consts::PI;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
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

#[derive(Debug, Clone, Copy)]
struct Matrix2D {
    data: [[f64; 2]; 2],
}

impl Matrix2D {
    fn new(data: [[f64; 2]; 2]) -> Self {
        Matrix2D { data }
    }

    fn identity() -> Self {
        Matrix2D {
            data: [[1.0, 0.0], [0.0, 1.0]],
        }
    }

    fn zeros() -> Self {
        Matrix2D {
            data: [[0.0, 0.0], [0.0, 0.0]],
        }
    }

    fn transpose(self) -> Self {
        Matrix2D {
            data: [[self.data[0][0], self.data[1][0]], [self.data[0][1], self.data[1][1]]],
        }
    }
}

impl Mul for Matrix2D {
    type Output = Matrix2D;

    fn mul(self, other: Matrix2D) -> Matrix2D {
        let mut result = Matrix2D::zeros();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}

impl Mul<Vector2D> for Matrix2D {
    type Output = Vector2D;

    fn mul(self, vector: Vector2D) -> Vector2D {
        Vector2D {
            x: self.data[0][0] * vector.x + self.data[0][1] * vector.y,
            y: self.data[1][0] * vector.x + self.data[1][1] * vector.y,
        }
    }
}

impl Mul<f64> for Matrix2D {
    type Output = Matrix2D;

    fn mul(self, scalar: f64) -> Matrix2D {
        Matrix2D {
            data: [[self.data[0][0] * scalar, self.data[0][1] * scalar],
                   [self.data[1][0] * scalar, self.data[1][1] * scalar]],
        }
    }
}

impl Matrix2D {
    fn inverse(self) -> Self {
        let determinant = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        Matrix2D {
            data: [[self.data[1][1] / determinant, -self.data[0][1] / determinant],
                   [-self.data[1][0] / determinant, self.data[0][0] / determinant]],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct KalmanFilter2D {
    state: Vector2D,
    error_covariance: Matrix2D,
    process_noise: f64,
    measurement_noise: f64,
    time_step: f64,
}

impl KalmanFilter2D {
    fn new(
        initial_position: Vector2D,
        initial_velocity: Vector2D,
        process_noise: f64,
        measurement_noise: f64,
        time_step: f64,
    ) -> Self {
        KalmanFilter2D {
            state: initial_position + initial_velocity * time_step,
            error_covariance: Matrix2D::identity(),
            process_noise,
            measurement_noise,
            time_step,
        }
    }

    fn predict(&mut self) {
        let state_transition_matrix = Matrix2D {
            data: [[1.0, self.time_step], [0.0, 1.0]],
        };
        let process_noise_matrix = Matrix2D {
            data: [[0.5 * self.process_noise * self.time_step.powi(2), 0.0],
                   [0.0, self.process_noise * self.time_step]],
        };

        self.state = state_transition_matrix * self.state;
        self.error_covariance =
            state_transition_matrix * self.error_covariance * state_transition_matrix.transpose()
                + process_noise_matrix;
    }

    fn update(&mut self, measurement: Vector2D) {
        let measurement_matrix = Matrix2D {
            data: [[1.0, 0.0], [0.0, 1.0]],
        };
        let measurement_noise_matrix = Matrix2D {
            data: [[self.measurement_noise, 0.0], [0.0, self.measurement_noise]],
        };

        let innovation = measurement - measurement_matrix * self.state;
        let innovation_covariance =
            measurement_matrix * self.error_covariance * measurement_matrix.transpose()
                + measurement_noise_matrix;

        let gain = self.error_covariance * measurement_matrix.transpose()
            * innovation_covariance.inverse();

        self.state = self.state + gain * innovation;
        self.error_covariance = (Matrix2D::identity() - gain * measurement_matrix)
            * self.error_covariance;
    }

    fn get_state(&self) -> Vector2D {
        self.state
    }
}