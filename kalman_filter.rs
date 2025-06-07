

use std::f64;

struct KalmanFilter2D {
    state: [f64; 2],
    velocity: [f64; 2],
    process_noise: f64,
    measurement_noise: f64,
    error_covariance: [[f64; 2]; 2],
}

impl KalmanFilter2D {
    fn new(state: [f64; 2], velocity: [f64; 2], process_noise: f64, measurement_noise: f64) -> Result<Self, String> {
        if process_noise < 0.0 || measurement_noise < 0.0 {
            return Err("Noise values cannot be negative".to_string());
        }
        Ok(KalmanFilter2D {
            state,
            velocity,
            process_noise,
            measurement_noise,
            error_covariance: [[1.0, 0.0], [0.0, 1.0]],
        })
    }

    fn predict(&mut self) {
        self.state[0] += self.velocity[0];
        self.state[1] += self.velocity[1];
        self.error_covariance[0][0] += self.process_noise;
        self.error_covariance[1][1] += self.process_noise;
    }

    fn update(&mut self, measurement: [f64; 2]) {
        let innovation = [measurement[0] - self.state[0], measurement[1] - self.state[1]];
        let innovation_covariance = [
            [self.error_covariance[0][0] + self.measurement_noise, self.error_covariance[0][1]],
            [self.error_covariance[1][0], self.error_covariance[1][1] + self.measurement_noise],
        ];
        let gain = [
            [
                (innovation_covariance[0][0] * innovation[0] + innovation_covariance[0][1] * innovation[1])
                    / (innovation_covariance[0][0] * innovation_covariance[0][0]
                        + innovation_covariance[0][1] * innovation_covariance[1][0]
                        + innovation_covariance[1][0] * innovation_covariance[0][1]
                        + innovation_covariance[1][1] * innovation_covariance[1][1]),
                (innovation_covariance[0][0] * innovation[0] + innovation_covariance[0][1] * innovation[1])
                    / (innovation_covariance[0][0] * innovation_covariance[0][0]
                        + innovation_covariance[0][1] * innovation_covariance[1][0]
                        + innovation_covariance[1][0] * innovation_covariance[0][1]
                        + innovation_covariance[1][1] * innovation_covariance[1][1]),
            ],
            [
                (innovation_covariance[1][0] * innovation[0] + innovation_covariance[1][1] * innovation[1])
                    / (innovation_covariance[0][0] * innovation_covariance[0][0]
                        + innovation_covariance[0][1] * innovation_covariance[1][0]
                        + innovation_covariance[1][0] * innovation_covariance[0][1]
                        + innovation_covariance[1][1] * innovation_covariance[1][1]),
                (innovation_covariance[1][0] * innovation[0] + innovation_covariance[1][1] * innovation[1])
                    / (innovation_covariance[0][0] * innovation_covariance[0][0]
                        + innovation_covariance[0][1] * innovation_covariance[1][0]
                        + innovation_covariance[1][0] * innovation_covariance[0][1]
                        + innovation_covariance[1][1] * innovation_covariance[1][1]),
            ],
        ];
        self.state[0] += gain[0][0] * innovation[0] + gain[0][1] * innovation[1];
        self.state[1] += gain[1][0] * innovation[0] + gain[1][1] * innovation[1];
        self.error_covariance[0][0] = (1.0 - gain[0][0]) * self.error_covariance[0][0] + gain[0][1] * self.error_covariance[1][0];
        self.error_covariance[0][1] = (1.0 - gain[0][0]) * self.error_covariance[0][1] + gain[0][1] * self.error_covariance[1][1];
        self.error_covariance[1][0] = (1.0 - gain[1][0]) * self.error_covariance[0][0] + gain[1][1] * self.error_covariance[1][0];
        self.error_covariance[1][1] = (1.0 - gain[1][0]) * self.error_covariance[0][1] + gain[1][1] * self.error_covariance[1][1];
    }

    fn get_state(&self) -> [f64; 2] {
        self.state
    }

    fn get_velocity(&self) -> [f64; 2] {
        self.velocity
    }
}