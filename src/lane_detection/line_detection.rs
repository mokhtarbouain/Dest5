use opencv::{
    core::{Point, Point2f, Vec4f},
    imgproc::{self, LineSegmentPoint},
    prelude::*,
};
use std::f64::consts::PI;

pub fn detect_lines(edges: &Mat) -> Result<Vec<Vec4f>, opencv::Error> {
    let mut lines = Mat::default();
    imgproc::hough_lines_p(
        edges,
        &mut lines,
        2.0,                // rho: distance resolution in pixels
        PI / 180.0,         // theta: angle resolution in radians
        15,                 // threshold: minimum number of votes
        10.0,               // min_line_length: minimum length of line
        20.0,               // max_line_gap: maximum allowed gap between line segments
    )?;

    let mut line_segments = Vec::new();
    for i in 0..lines.rows() {
        let line = lines.at_row::<Vec4f>(i)?;
        line_segments.push(*line);
    }

    Ok(line_segments)
}

pub fn filter_lines(lines: &[Vec4f]) -> Vec<Vec4f> {
    lines
        .iter()
        .filter_map(|line| {
            let x1 = line[0] as f64;
            let y1 = line[1] as f64;
            let x2 = line[2] as f64;
            let y2 = line[3] as f64;

            let dx = x2 - x1;
            let dy = y2 - y1;

            // Handle vertical lines
            let slope = if dx.abs() < 1e-6 { 999.0 } else { dy / dx };

            // Filter by slope magnitude
            if slope.abs() > 0.5 {
                Some(*line)
            } else {
                None
            }
        })
        .collect()
}

pub fn classify_lines(lines: &[Vec4f], image_width: i32) -> (Vec<Vec4f>, Vec<Vec4f>) {
    let mid_x = image_width as f64 / 2.0;
    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();

    for line in lines {
        let x1 = line[0] as f64;
        let y1 = line[1] as f64;
        let x2 = line[2] as f64;
        let y2 = line[3] as f64;

        let dx = x2 - x1;
        let dy = y2 - y1;

        // Handle vertical lines
        let slope = if dx.abs() < 1e-6 { 999.0 } else { dy / dx };
        
        // Calculate midpoint of the line
        let mid_point_x = (x1 + x2) / 2.0;

        // Classify based on slope and position
        if slope < 0.0 && mid_point_x < mid_x {
            left_lines.push(*line);
        } else if slope > 0.0 && mid_point_x > mid_x {
            right_lines.push(*line);
        }
    }

    (left_lines, right_lines)
}

pub fn fit_lane_line(lines: &[Vec4f]) -> Option<(f64, f64)> {
    if lines.is_empty() {
        return None;
    }

    let mut points = Vec::new();
    for line in lines {
        points.push((line[0] as f64, line[1] as f64));
        points.push((line[2] as f64, line[3] as f64));
    }

    let n = points.len() as f64;
    
    // Calculate means
    let mean_x: f64 = points.iter().map(|(x, _)| x).sum::<f64>() / n;
    let mean_y: f64 = points.iter().map(|(_, y)| y).sum::<f64>() / n;
    
    // Calculate sums for linear regression
    let mut ss_xy = 0.0;
    let mut ss_xx = 0.0;
    
    for (x, y) in &points {
        ss_xy += (x - mean_x) * (y - mean_y);
        ss_xx += (x - mean_x) * (x - mean_x);
    }
    
    // Avoid division by zero
    if ss_xx.abs() < 1e-6 {
        return None;
    }
    
    // Calculate slope and intercept
    let slope = ss_xy / ss_xx;
    let intercept = mean_y - (slope * mean_x);
    
    Some((slope, intercept))
}

pub fn calculate_lane_points(
    source: &Mat,
    slope: f64,
    intercept: f64,
) -> Option<(Point, Point)> {
    let rows = source.rows();
    
    // Bottom of the image
    let y1 = rows as f64;
    let x1 = (y1 - intercept) / slope;
    
    // 40% from the top of the image
    let y2 = rows as f64 * (1.0 - 0.4);
    let x2 = (y2 - intercept) / slope;
    
    // Check if points are within image bounds
    if x1.is_finite() && x2.is_finite() {
        Some((
            Point::new(x1 as i32, y1 as i32),
            Point::new(x2 as i32, y2 as i32),
        ))
    } else {
        None
    }
}

pub fn validate_lanes(left_lane: Option<(Point, Point)>, right_lane: Option<(Point, Point)>) -> bool {
    match (left_lane, right_lane) {
        (Some((left_bottom, left_top)), Some((right_bottom, right_top))) => {
            // Check if right lane is to the right of left lane
            right_top.x > left_top.x && right_bottom.x > left_bottom.x
        }
        _ => false,
    }
}