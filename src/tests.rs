

use kalman_filter::{KalmanFilter, Vector2D};
use std::f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kalman_filter() {
        let mut kf = KalmanFilter::new(0.1, 0.1, 0.1, 0.1);
        let z = Vector2D::new(1.0, 2.0);
        let x = kf.predict();
        kf.update(z);
        assert_eq!(x.x, 0.0);
        assert_eq!(x.y, 0.0);
    }

    #[test]
    fn test_kalman_filter_edge_case() {
        let mut kf = KalmanFilter::new(0.0, 0.0, 0.0, 0.0);
        let z = Vector2D::new(1.0, 2.0);
        let x = kf.predict();
        kf.update(z);
        assert_eq!(x.x, 0.0);
        assert_eq!(x.y, 0.0);
    }

    #[test]
    fn test_kalman_filter_error_handling() {
        let mut kf = KalmanFilter::new(0.1, 0.1, 0.1, 0.1);
        let z = Vector2D::new(f64::NAN, 2.0);
        let x = kf.predict();
        kf.update(z);
        assert!(x.x.is_nan());
        assert_eq!(x.y, 0.0);
    }

    #[test]
    fn test_kalman_filter_predict() {
        let mut kf = KalmanFilter::new(0.1, 0.1, 0.1, 0.1);
        let x = kf.predict();
        assert_eq!(x.x, 0.0);
        assert_eq!(x.y, 0.0);
    }

    #[test]
    fn test_kalman_filter_update() {
        let mut kf = KalmanFilter::new(0.1, 0.1, 0.1, 0.1);
        let z = Vector2D::new(1.0, 2.0);
        kf.update(z);
        let x = kf.predict();
        assert_eq!(x.x, 1.0);
        assert_eq!(x.y, 2.0);
    }

    #[test]
    fn test_vector2d() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 6.0);
    }

    #[test]
    fn test_vector2d_scalar_multiplication() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
    }

    #[test]
    fn test_vector2d_transpose() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = v1.transpose();
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 2.0);
    }

    #[test]
    fn test_vector2d_dot_product() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let dot_product = v1.dot(v2);
        assert_eq!(dot_product, 11.0);
    }

    #[test]
    fn test_vector2d_magnitude() {
        let v1 = Vector2D::new(3.0, 4.0);
        let magnitude = v1.magnitude();
        assert_eq!(magnitude, 5.0);
    }

    #[test]
    fn test_vector2d_normalize() {
        let v1 = Vector2D::new(3.0, 4.0);
        let v2 = v1.normalize();
        assert_eq!(v2.x, 0.6);
        assert_eq!(v2.y, 0.8);
    }

    #[test]
    fn test_vector2d_normalize_edge_case() {
        let v1 = Vector2D::new(0.0, 0.0);
        let v2 = v1.normalize();
        assert!(v2.x.is_nan());
        assert!(v2.y.is_nan());
    }

    #[test]
    fn test_vector2d_normalize_error_handling() {
        let v1 = Vector2D::new(f64::NAN, 4.0);
        let v2 = v1.normalize();
        assert!(v2.x.is_nan());
        assert!(v2.y.is_nan());
    }
}