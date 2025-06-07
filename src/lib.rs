

mod kalman_filter_2d;

pub use kalman_filter_2d::KalmanFilter2D;

mod kalman_filter_2d {
    use std::f64;

    pub struct KalmanFilter2D {
        q: f64,
        r: f64,
        x: f64,
        y: f64,
        p_xx: f64,
        p_xy: f64,
        p_yx: f64,
        p_yy: f64,
    }

    impl KalmanFilter2D {
        pub fn new(q: f64, r: f64, x: f64, y: f64) -> KalmanFilter2D {
            KalmanFilter2D {
                q,
                r,
                x,
                y,
                p_xx: 1.0,
                p_xy: 0.0,
                p_yx: 0.0,
                p_yy: 1.0,
            }
        }

        pub fn predict(&mut self, u: f64, v: f64) {
            self.x += u;
            self.y += v;
            self.p_xx += self.q;
            self.p_xy += 0.0;
            self.p_yx += 0.0;
            self.p_yy += self.q;
        }

        pub fn update(&mut self, z_x: f64, z_y: f64) {
            let k_x = self.p_xx * self.r / (self.r * self.r + self.p_xx * self.p_xx);
            let k_y = self.p_yy * self.r / (self.r * self.r + self.p_yy * self.p_yy);
            self.x += k_x * (z_x - self.x);
            self.y += k_y * (z_y - self.y);
            self.p_xx = (1.0 - k_x * self.r) * self.p_xx;
            self.p_xy = (1.0 - k_x * self.r) * self.p_xy;
            self.p_yx = (1.0 - k_y * self.r) * self.p_yx;
            self.p_yy = (1.0 - k_y * self.r) * self.p_yy;
        }

        pub fn get_state(&self) -> (f64, f64) {
            (self.x, self.y)
        }
    }
}