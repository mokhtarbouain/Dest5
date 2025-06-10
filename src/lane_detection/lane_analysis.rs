use opencv::{
    core::{Mat, Point, Scalar, Vec4f},
    imgproc::{hough_lines_p, line},
    prelude::*,
};
use std::f64::consts::PI;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LaneDetectionError {
    #[error("Lane analysis error: {0}")]
    LaneAnalysisError(String),
    #[error("OpenCV error: {0}")]
    OpenCvError(#[from] opencv::Error),
}

type Result<T> = std::result::Result<T, LaneDetectionError>;

pub fn hough_lines(edge_map: &Mat) -> Result<Vec<(f32, f32, f32, f32)>> {
    let mut lines = Mat::default();
    hough_lines_p(
        edge_map,
        &mut lines,
        1.0,
        PI / 180.0,
        50,
        50.0,
        10.0,
    )?;

    let mut result = Vec::new();
    for i in 0..lines.rows() {
        let line = lines.at_row::<Vec4f>(i)?;
        result.push((line[0], line[1], line[2], line[3]));
    }

    Ok(result)
}

pub fn separate_lines(
    lines: &[(f32, f32, f32, f32)],
) -> Result<(Vec<(f32, f32, f32, f32)>, Vec<(f32, f32, f32, f32)>)> {
    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();

    for &line in lines {
        let (x1, y1, x2, y2) = line;
        
        // Avoid division by zero
        if x2 == x1 {
            continue;
        }
        
        let slope = (y2 - y1) / (x2 - x1);
        
        // Filter out horizontal lines
        if slope.abs() < 0.1 {
            continue;
        }
        
        if slope < 0.0 {
            left_lines.push(line);
        } else {
            right_lines.push(line);
        }
    }

    Ok((left_lines, right_lines))
}

pub fn average_slope_intercept(
    left_lines: &[(f32, f32, f32, f32)],
    right_lines: &[(f32, f32, f32, f32)],
) -> Result<((f32, f32), (f32, f32))> {
    let calculate_average = |lines: &[(f32, f32, f32, f32)]| -> Result<(f32, f32)> {
        if lines.is_empty() {
            return Err(LaneDetectionError::LaneAnalysisError(
                "No lines detected".to_string(),
            ));
        }

        let mut x_points = Vec::new();
        let mut y_points = Vec::new();

        for &(x1, y1, x2, y2) in lines {
            x_points.push(x1 as f64);
            x_points.push(x2 as f64);
            y_points.push(y1 as f64);
            y_points.push(y2 as f64);
        }

        // Simple linear regression
        let n = x_points.len() as f64;
        let sum_x: f64 = x_points.iter().sum();
        let sum_y: f64 = y_points.iter().sum();
        let sum_xy: f64 = x_points.iter().zip(y_points.iter()).map(|(&x, &y)| x * y).sum();
        let sum_xx: f64 = x_points.iter().map(|&x| x * x).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        Ok((slope as f32, intercept as f32))
    };

    let left_params = calculate_average(left_lines)?;
    let right_params = calculate_average(right_lines)?;

    Ok((left_params, right_params))
}

pub fn draw_lanes(
    image: &Mat,
    left_params: (f32, f32),
    right_params: (f32, f32),
) -> Result<Mat> {
    let mut result = image.clone();
    let (height, width, _) = (image.rows(), image.cols(), image.channels());
    
    let y_bottom = height as f32;
    let y_horizon = height as f32 * 0.6; // Horizon at 60% of the image height
    
    let (left_slope, left_intercept) = left_params;
    let (right_slope, right_intercept) = right_params;
    
    // Calculate x coordinates at the bottom and horizon for left lane
    let left_x_bottom = (y_bottom - left_intercept) / left_slope;
    let left_x_horizon = (y_horizon - left_intercept) / left_slope;
    
    // Calculate x coordinates at the bottom and horizon for right lane
    let right_x_bottom = (y_bottom - right_intercept) / right_slope;
    let right_x_horizon = (y_horizon - right_intercept) / right_slope;
    
    // Draw left lane line
    line(
        &mut result,
        Point::new(left_x_bottom as i32, y_bottom as i32),
        Point::new(left_x_horizon as i32, y_horizon as i32),
        Scalar::new(0.0, 0.0, 255.0, 0.0), // Red color
        5,
        opencv::imgproc::LINE_AA,
        0,
    )?;
    
    // Draw right lane line
    line(
        &mut result,
        Point::new(right_x_bottom as i32, y_bottom as i32),
        Point::new(right_x_horizon as i32, y_horizon as i32),
        Scalar::new(0.0, 0.0, 255.0, 0.0), // Red color
        5,
        opencv::imgproc::LINE_AA,
        0,
    )?;
    
    Ok(result)
}