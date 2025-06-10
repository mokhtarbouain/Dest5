pub const CANNY_LOW_THRESHOLD: f64 = 50.0;
pub const CANNY_HIGH_THRESHOLD: f64 = 150.0;
pub const GAUSSIAN_KERNEL_SIZE: i32 = 5;

pub const HOUGH_RHO: f32 = 1.0;
pub const HOUGH_THETA: f32 = std::f32::consts::PI / 180.0;
pub const HOUGH_THRESHOLD: i32 = 50;
pub const HOUGH_MIN_LINE_LENGTH: f64 = 50.0;
pub const HOUGH_MAX_LINE_GAP: f64 = 10.0;

pub const PI: f32 = std::f32::consts::PI;

pub const WHITE_LOWER: [u8; 3] = [200, 200, 200];
pub const WHITE_UPPER: [u8; 3] = [255, 255, 255];

pub const YELLOW_LOWER: [u8; 3] = [20, 100, 100];
pub const YELLOW_UPPER: [u8; 3] = [30, 255, 255];

pub const DAY_BRIGHTNESS_THRESHOLD: f32 = 100.0;