use opencv::{
    core::{Point, Point2f, Size, Vec3b},
    prelude::*,
};
use thiserror::Error;
use std::convert::From;

// Constants for image dimensions
pub const IMAGE_WIDTH: i32 = 640;
pub const IMAGE_HEIGHT: i32 = 480;

// Region of interest coordinates (polygon vertices)
pub const ROI_VERTICES: [Point; 4] = [
    Point::new(0, IMAGE_HEIGHT),
    Point::new(IMAGE_WIDTH / 3, IMAGE_HEIGHT / 2),
    Point::new(2 * IMAGE_WIDTH / 3, IMAGE_HEIGHT / 2),
    Point::new(IMAGE_WIDTH, IMAGE_HEIGHT),
];

// Thresholds for edge detection
pub const CANNY_LOW_THRESHOLD: f64 = 50.0;
pub const CANNY_HIGH_THRESHOLD: f64 = 150.0;
pub const CANNY_APERTURE_SIZE: i32 = 3;

// Thresholds for line detection (Hough transform parameters)
pub const HOUGH_RHO: f64 = 1.0;
pub const HOUGH_THETA: f64 = std::f64::consts::PI / 180.0;
pub const HOUGH_THRESHOLD: i32 = 20;
pub const HOUGH_MIN_LINE_LENGTH: f64 = 20.0;
pub const HOUGH_MAX_LINE_GAP: f64 = 30.0;

// Color filtering thresholds for yellow lanes
pub const YELLOW_LOWER: Vec3b = Vec3b::new(20, 100, 100);
pub const YELLOW_UPPER: Vec3b = Vec3b::new(30, 255, 255);

// Color filtering thresholds for white lanes
pub const WHITE_LOWER: Vec3b = Vec3b::new(0, 0, 200);
pub const WHITE_UPPER: Vec3b = Vec3b::new(255, 30, 255);

// Gaussian blur parameters
pub const GAUSSIAN_KERNEL_SIZE: Size = Size::new(5, 5);
pub const GAUSSIAN_SIGMA_X: f64 = 0.0;

// Lane detection error types
#[derive(Error, Debug)]
pub enum LaneDetectionError {
    #[error("OpenCV error: {0}")]
    OpenCvError(String),

    #[error("Image processing error: {0}")]
    ImageProcessingError(String),

    #[error("Lane analysis error: {0}")]
    LaneAnalysisError(String),

    #[error("Regression error: {0}")]
    RegressionError(String),
}

impl From<opencv::Error> for LaneDetectionError {
    fn from(error: opencv::Error) -> Self {
        LaneDetectionError::OpenCvError(error.to_string())
    }
}

pub fn image_to_roi_coordinates(point: Point, roi_offset: Point) -> Point {
    Point::new(point.x - roi_offset.x, point.y - roi_offset.y)
}

pub fn roi_to_image_coordinates(point: Point, roi_offset: Point) -> Point {
    Point::new(point.x + roi_offset.x, point.y + roi_offset.y)
}

pub fn calculate_distance(p1: Point, p2: Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    ((dx * dx + dy * dy) as f64).sqrt()
}

pub fn calculate_angle(p1: Point, p2: Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dy as f64).atan2(dx as f64) * 180.0 / std::f64::consts::PI
}

pub fn line_intersection(line1: (Point, Point), line2: (Point, Point)) -> Option<Point2f> {
    let x1 = line1.0.x as f32;
    let y1 = line1.0.y as f32;
    let x2 = line1.1.x as f32;
    let y2 = line1.1.y as f32;
    
    let x3 = line2.0.x as f32;
    let y3 = line2.0.y as f32;
    let x4 = line2.1.x as f32;
    let y4 = line2.1.y as f32;
    
    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    
    if denominator.abs() < f32::EPSILON {
        return None; // Lines are parallel
    }
    
    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
    
    let x = x1 + ua * (x2 - x1);
    let y = y1 + ua * (y2 - y1);
    
    Some(Point2f::new(x, y))
}

pub fn slope(p1: Point, p2: Point) -> f64 {
    if p2.x - p1.x == 0 {
        return f64::MAX; // Vertical line
    }
    (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64
}

pub fn y_intercept(point: Point, slope: f64) -> f64 {
    point.y as f64 - slope * point.x as f64
}

pub fn point_at_y(slope: f64, y_intercept: f64, y: i32) -> Point {
    let x = ((y as f64 - y_intercept) / slope) as i32;
    Point::new(x, y)
}

pub fn is_point_in_image(point: Point) -> bool {
    point.x >= 0 && point.x < IMAGE_WIDTH && point.y >= 0 && point.y < IMAGE_HEIGHT
}