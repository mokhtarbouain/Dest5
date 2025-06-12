use opencv::{
    core::{bitwise_and, mean, Mat, Scalar, CV_8UC3},
    imgproc::{cvt_color, COLOR_BGR2HSV},
    prelude::*,
};

/// Detects whether the frame is in day or night lighting conditions
pub fn detect_lighting_condition(frame: &Mat) -> bool {
    // Calculate mean pixel value
    let mean_val = mean(&frame, &Mat::default()).unwrap();
    
    // Define threshold for night detection
    // If mean brightness is below threshold, consider it night
    let night_threshold = 80.0;
    
    // Return true if night, false if day
    mean_val[0] < night_threshold || mean_val[1] < night_threshold || mean_val[2] < night_threshold
}

/// Filters white color from the frame
pub fn filter_white_color(frame: &Mat) -> Mat {
    let mut white_mask = Mat::default();
    let mut white_filtered = Mat::default();
    
    // Define RGB thresholds for white color
    let lower_white = Scalar::new(200.0, 200.0, 200.0, 0.0);
    let upper_white = Scalar::new(255.0, 255.0, 255.0, 0.0);
    
    // Create mask for white pixels
    opencv::core::in_range(&frame, &lower_white, &upper_white, &mut white_mask).unwrap();
    
    // Extract white regions using bitwise operation
    bitwise_and(&frame, &frame, &mut white_filtered, &white_mask).unwrap();
    
    white_filtered
}

/// Filters yellow color from the frame
pub fn filter_yellow_color(frame: &Mat) -> Mat {
    let mut hsv = Mat::default();
    let mut yellow_mask = Mat::default();
    let mut yellow_filtered = Mat::default();
    
    // Convert frame to HSV color space
    cvt_color(&frame, &mut hsv, COLOR_BGR2HSV, 0).unwrap();
    
    // Define HSV thresholds for yellow color
    let lower_yellow = Scalar::new(20.0, 100.0, 100.0, 0.0);
    let upper_yellow = Scalar::new(30.0, 255.0, 255.0, 0.0);
    
    // Create mask for yellow pixels
    opencv::core::in_range(&hsv, &lower_yellow, &upper_yellow, &mut yellow_mask).unwrap();
    
    // Extract yellow regions using bitwise operation
    bitwise_and(&frame, &frame, &mut yellow_filtered, &yellow_mask).unwrap();
    
    yellow_filtered
}

/// Filters gray color from the frame
pub fn filter_gray_color(frame: &Mat) -> Mat {
    let mut gray_mask = Mat::default();
    let mut gray_filtered = Mat::default();
    
    // Define RGB thresholds for gray color
    let lower_gray = Scalar::new(100.0, 100.0, 100.0, 0.0);
    let upper_gray = Scalar::new(150.0, 150.0, 150.0, 0.0);
    
    // Create mask for gray pixels
    opencv::core::in_range(&frame, &lower_gray, &upper_gray, &mut gray_mask).unwrap();
    
    // Extract gray regions using bitwise operation
    bitwise_and(&frame, &frame, &mut gray_filtered, &gray_mask).unwrap();
    
    gray_filtered
}

/// Combines filtered colors based on lighting conditions
pub fn combine_filtered_colors(white_filtered: &Mat, yellow_filtered: &Mat, gray_filtered: &Mat, is_night: bool) -> Mat {
    let mut combined = Mat::new_rows_cols_with_default(
        white_filtered.rows(),
        white_filtered.cols(),
        CV_8UC3,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    ).unwrap();
    
    // Add white and yellow with equal weights
    opencv::core::add_weighted(white_filtered, 1.0, yellow_filtered, 1.0, 0.0, &mut combined, -1).unwrap();
    
    // If night, add gray filtered image
    if is_night {
        opencv::core::add_weighted(&combined, 1.0, gray_filtered, 1.0, 0.0, &mut combined, -1).unwrap();
    }
    
    combined
}