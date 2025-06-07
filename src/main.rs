

use rand::Rng;
use std::f64;

struct KalmanFilter {
    q: f64,
    r: f64,
    x: f64,
    p: f64,
    k: f64,
}

impl KalmanFilter {
    fn new(q: f64, r: f64, x: f64, p: f64) -> KalmanFilter {
        KalmanFilter { q, r, x, p, k: 0.0 }
    }

    fn predict(&mut self, dt: f64) {
        if dt < 0.0 {
            panic!("dt must be non-negative");
        }
        self.p += dt * self.q;
    }

    fn update(&mut self, measurement: f64) {
        if self.r <= 0.0 {
            panic!("measurement noise must be positive");
        }
        self.k = self.p / (self.p + self.r);
        self.x = self.x + self.k * (measurement - self.x);
        self.p = (1.0 - self.k) * self.p;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut kf = KalmanFilter::new(0.1, 0.5, 0.0, 1.0);
    let measurements = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let dt = 1.0;

    if measurements.is_empty() {
        println!("Measurements vector is empty");
        return;
    }

    if dt <= 0.0 {
        println!("dt must be positive");
        return;
    }

    if kf.q <= 0.0 {
        println!("process noise must be positive");
        return;
    }

    if kf.r <= 0.0 {
        println!("measurement noise must be positive");
        return;
    }

    for measurement in measurements {
        kf.predict(dt);
        kf.update(measurement);
        println!("Estimated state: {}", kf.x);
    }
}