use anyhow::{Result, anyhow};
use opencv::{
    core::{Mat, Point, Scalar, mean, CV_8UC3, CV_8UC1},
    imgproc::{cvt_color, COLOR_BGR2GRAY, line, hough_lines_p},
    prelude::MatTraitConst,
};

use crate::utils::linear_regression;

const DAY_BRIGHTNESS_THRESHOLD: f64 = 80.0;
const HOUGH_RHO: f64 = 1.0;
const HOUGH_THETA: f64 = std::f64::consts::PI / 180.0;
const HOUGH_THRESHOLD: i32 = 50;
const HOUGH_MIN_LINE_LENGTH: f64 = 50.0;
const HOUGH_MAX_LINE_GAP: f64 = 10.0;

pub fn is_day_time(image: &Mat) -> Result<bool> {
    let mut gray = Mat::default();
    cvt_color(image, &mut gray, COLOR_BGR2GRAY, 0)?;
    
    let mean_val = mean(&gray, &Mat::default())?;
    let brightness = mean_val[0];
    
    Ok(brightness > DAY_BRIGHTNESS_THRESHOLD)
}

pub fn hough_lines(image: &Mat) -> Result<Mat> {
    let mut lines = Mat::default();
    hough_lines_p(
        image,
        &mut lines,
        HOUGH_RHO,
        HOUGH_THETA,
        HOUGH_THRESHOLD,
        HOUGH_MIN_LINE_LENGTH,
        HOUGH_MAX_LINE_GAP,
    )?;
    
    Ok(lines)
}

pub fn draw_lanes(image: &Mat, lines: &Mat) -> Result<Mat> {
    if lines.empty() {
        return Err(anyhow!("No lines detected"));
    }
    
    let mut result = Mat::new_rows_cols_with_default(
        image.rows(),
        image.cols(),
        CV_8UC3,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    )?;
    
    image.copy_to(&mut result)?;
    
    let mut left_lane_points: Vec<(f64, f64)> = Vec::new();
    let mut right_lane_points: Vec<(f64, f64)> = Vec::new();
    
    let lines_array = lines.to_vec_2d::<i32>()?;
    
    for line in lines_array {
        if line.len() < 4 {
            continue;
        }
        
        let x1 = line[0] as f64;
        let y1 = line[1] as f64;
        let x2 = line[2] as f64;
        let y2 = line[3] as f64;
        
        if x2 == x1 {
            continue; // Skip vertical lines to avoid division by zero
        }
        
        let slope = (y2 - y1) / (x2 - x1);
        let abs_slope = slope.abs();
        
        // Filter out horizontal lines
        if abs_slope < 0.5 {
            continue;
        }
        
        // Separate left and right lanes based on slope
        if slope < 0.0 {
            // Left lane (negative slope)
            left_lane_points.push((x1, y1));
            left_lane_points.push((x2, y2));
        } else {
            // Right lane (positive slope)
            right_lane_points.push((x1, y1));
            right_lane_points.push((x2, y2));
        }
    }
    
    // Draw left lane
    if !left_lane_points.is_empty() {
        let (slope, intercept) = linear_regression(&left_lane_points)?;
        let y_bottom = image.rows() as f64;
        let y_top = image.rows() as f64 * 0.6; // Start from 60% of the image height
        
        let x_bottom = (y_bottom - intercept) / slope;
        let x_top = (y_top - intercept) / slope;
        
        line(
            &mut result,
            Point::new(x_bottom as i32, y_bottom as i32),
            Point::new(x_top as i32, y_top as i32),
            Scalar::new(0.0, 0.0, 255.0, 0.0), // Red color
            3,
            8,
            0,
        )?;
    }
    
    // Draw right lane
    if !right_lane_points.is_empty() {
        let (slope, intercept) = linear_regression(&right_lane_points)?;
        let y_bottom = image.rows() as f64;
        let y_top = image.rows() as f64 * 0.6; // Start from 60% of the image height
        
        let x_bottom = (y_bottom - intercept) / slope;
        let x_top = (y_top - intercept) / slope;
        
        line(
            &mut result,
            Point::new(x_bottom as i32, y_bottom as i32),
            Point::new(x_top as i32, y_top as i32),
            Scalar::new(0.0, 0.0, 255.0, 0.0), // Red color
            3,
            8,
            0,
        )?;
    }
    
    Ok(result)
}