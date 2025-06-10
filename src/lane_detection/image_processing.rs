use opencv::{
    core::{Mat, Point, Scalar, Vector, CV_8UC1},
    imgproc::{canny, cvt_color, fill_poly, gaussian_blur, COLOR_BGR2GRAY, COLOR_BGR2HSV},
    prelude::*,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LaneDetectionError {
    #[error("OpenCV error: {0}")]
    OpenCvError(#[from] opencv::Error),
    
    #[error("Image processing error: {0}")]
    ImageProcessingError(String),
}

pub fn is_day_time(image: &Mat) -> Result<bool, LaneDetectionError> {
    let mut gray = Mat::default();
    cvt_color(&image, &mut gray, COLOR_BGR2GRAY, 0)?;
    
    let mean = opencv::core::mean(&gray, &Mat::default())?;
    
    // Threshold for day/night detection
    // If the average brightness is above 80, consider it daytime
    Ok(mean[0] > 80.0)
}

pub fn region_of_interest(image: &Mat) -> Result<Mat, LaneDetectionError> {
    let height = image.rows();
    let width = image.cols();
    
    let mut mask = Mat::new_rows_cols(height, width, CV_8UC1)?;
    mask.set_scalar(Scalar::new(0.0, 0.0, 0.0, 0.0))?;
    
    // Define a polygon for the region of interest
    let mut roi_points = Vector::<Point>::new();
    roi_points.push(Point::new(0, height));
    roi_points.push(Point::new(width / 3, height / 2));
    roi_points.push(Point::new(width * 2 / 3, height / 2));
    roi_points.push(Point::new(width, height));
    
    let roi_points_vec = Vector::from_slice(&[roi_points]);
    
    // Fill the polygon
    fill_poly(
        &mut mask,
        &roi_points_vec,
        Scalar::new(255.0, 255.0, 255.0, 255.0),
        opencv::imgproc::LINE_8,
        0,
        Point::new(0, 0),
    )?;
    
    // Apply the mask to the image
    let mut masked_image = Mat::default();
    opencv::core::bitwise_and(image, image, &mut masked_image, &mask)?;
    
    Ok(masked_image)
}

pub fn filter_colors(image: &Mat, is_daytime: bool) -> Result<Mat, LaneDetectionError> {
    let mut hsv = Mat::default();
    cvt_color(image, &mut hsv, COLOR_BGR2HSV, 0)?;
    
    let mut mask = Mat::default();
    
    if is_daytime {
        // Daytime: Filter for white and yellow colors
        let mut white_mask = Mat::default();
        let mut yellow_mask = Mat::default();
        
        // White color range
        opencv::core::in_range(
            &hsv,
            &Scalar::new(0.0, 0.0, 200.0, 0.0),
            &Scalar::new(180.0, 30.0, 255.0, 0.0),
            &mut white_mask,
        )?;
        
        // Yellow color range
        opencv::core::in_range(
            &hsv,
            &Scalar::new(20.0, 100.0, 100.0, 0.0),
            &Scalar::new(30.0, 255.0, 255.0, 0.0),
            &mut yellow_mask,
        )?;
        
        // Combine masks
        opencv::core::bitwise_or(&white_mask, &yellow_mask, &mut mask, &Mat::default())?;
    } else {
        // Nighttime: Adjust parameters for low-light conditions
        opencv::core::in_range(
            &hsv,
            &Scalar::new(0.0, 0.0, 150.0, 0.0),
            &Scalar::new(180.0, 60.0, 255.0, 0.0),
            &mut mask,
        )?;
    }
    
    // Apply the mask to the original image
    let mut filtered_image = Mat::default();
    opencv::core::bitwise_and(image, image, &mut filtered_image, &mask)?;
    
    Ok(filtered_image)
}

pub fn apply_grayscale(image: &Mat) -> Result<Mat, LaneDetectionError> {
    let mut gray_image = Mat::default();
    cvt_color(image, &mut gray_image, COLOR_BGR2GRAY, 0)?;
    Ok(gray_image)
}

pub fn apply_gaussian_blur(gray_image: &Mat) -> Result<Mat, LaneDetectionError> {
    let mut blurred_image = Mat::default();
    let kernel_size = 5;
    gaussian_blur(
        gray_image,
        &mut blurred_image,
        opencv::core::Size::new(kernel_size, kernel_size),
        0.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
    )?;
    Ok(blurred_image)
}

pub fn apply_canny(blurred_image: &Mat) -> Result<Mat, LaneDetectionError> {
    let mut edges = Mat::default();
    let low_threshold = 50;
    let high_threshold = 150;
    canny(
        blurred_image,
        &mut edges,
        low_threshold as f64,
        high_threshold as f64,
        3,
        false,
    )?;
    Ok(edges)
}