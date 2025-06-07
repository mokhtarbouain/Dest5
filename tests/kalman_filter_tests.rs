

use kalman_filter::{KalmanFilter2D, KalmanFilterOptions};
use std::f64;

#[cfg(test)]
mod kalman_filter_tests {
    use super::*;

    #[test]
    fn test_new_valid_input() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_ok());
    }

    #[test]
    fn test_new_invalid_input() {
        let options = KalmanFilterOptions {
            q: -0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_err());
    }

    #[test]
    fn test_new_invalid_input_q() {
        let options = KalmanFilterOptions {
            q: 0.0,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_err());
    }

    #[test]
    fn test_new_invalid_input_r() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: -0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_err());
    }

    #[test]
    fn test_new_invalid_input_p() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: -1.0,
            f: 1.0,
            h: 1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_err());
    }

    #[test]
    fn test_new_invalid_input_f() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: -1.0,
            h: 1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_err());
    }

    #[test]
    fn test_new_invalid_input_h() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: -1.0,
        };
        let filter = KalmanFilter2D::new(options);
        assert!(filter.is_err());
    }

    #[test]
    fn test_predict() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let mut filter = KalmanFilter2D::new(options).unwrap();
        let state = filter.predict(1.0);
        assert!(state.is_ok());
    }

    #[test]
    fn test_predict_zero() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let mut filter = KalmanFilter2D::new(options).unwrap();
        let state = filter.predict(0.0);
        assert!(state.is_ok());
    }

    #[test]
    fn test_predict_negative() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let mut filter = KalmanFilter2D::new(options).unwrap();
        let state = filter.predict(-1.0);
        assert!(state.is_ok());
    }

    #[test]
    fn test_update() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let mut filter = KalmanFilter2D::new(options).unwrap();
        let state = filter.update(1.0, 1.0);
        assert!(state.is_ok());
    }

    #[test]
    fn test_update_zero() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let mut filter = KalmanFilter2D::new(options).unwrap();
        let state = filter.update(0.0, 0.0);
        assert!(state.is_ok());
    }

    #[test]
    fn test_update_negative() {
        let options = KalmanFilterOptions {
            q: 0.1,
            r: 0.1,
            p: 1.0,
            f: 1.0,
            h: 1.0,
        };
        let mut filter = KalmanFilter2D::new(options).unwrap();
        let state = filter.update(-1.0, -1.0);
        assert!(state.is_ok());
    }
}