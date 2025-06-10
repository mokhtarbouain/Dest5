use opencv::{
    core::{Mat, Point, Scalar, CV_8UC3},
    imgproc::{line, HoughLinesP, LINE_AA},
    prelude::*,
};
use thiserror::Error;

use crate::lane_detection::constants;
use crate::lane_detection::regression::mean;

#[derive(Error, Debug)]
pub enum LineDetectionError {
    #[error("OpenCV error: {0}")]
    OpenCvError(#[from] opencv::Error),
    #[error("No lines detected")]
    NoLinesDetected,
}

pub struct Line {
    pub slope: f64,
    pub intercept: f64,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

pub fn hough_lines(edge_map: &Mat) -> Result<Vec<Line>, LineDetectionError> {
    let mut lines = Mat::default();
    HoughLinesP(
        edge_map,
        &mut lines,
        constants::HOUGH_RHO,
        constants::HOUGH_THETA,
        constants::HOUGH_THRESHOLD,
        constants::HOUGH_MIN_LINE_LENGTH,
        constants::HOUGH_MAX_LINE_GAP,
    )?;

    if lines.rows() == 0 {
        return Err(LineDetectionError::NoLinesDetected);
    }

    let mut result = Vec::new();
    for i in 0..lines.rows() {
        let line = lines.at_row::<i32>(i)?;
        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        let slope = if x2 != x1 {
            (y2 - y1) as f64 / (x2 - x1) as f64
        } else {
            std::f64::INFINITY
        };

        let intercept = y1 as f64 - slope * x1 as f64;

        result.push(Line {
            slope,
            intercept,
            x1,
            y1,
            x2,
            y2,
        });
    }

    Ok(result)
}

pub fn draw_lanes(
    image: &mut Mat,
    lines: &[Line],
) -> Result<(), LineDetectionError> {
    let height = image.rows();
    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();

    for line in lines {
        if line.slope.abs() < 0.5 {
            continue;
        }

        if line.slope < 0.0 {
            left_lines.push(line);
        } else {
            right_lines.push(line);
        }
    }

    if !left_lines.is_empty() {
        let left_lane = average_lane(&left_lines, height)?;
        draw_lane(image, &left_lane)?;
    }

    if !right_lines.is_empty() {
        let right_lane = average_lane(&right_lines, height)?;
        draw_lane(image, &right_lane)?;
    }

    Ok(())
}

fn average_lane(lines: &[&Line], height: i32) -> Result<Line, LineDetectionError> {
    let slopes: Vec<f64> = lines.iter().map(|line| line.slope).collect();
    let intercepts: Vec<f64> = lines.iter().map(|line| line.intercept).collect();

    let avg_slope = mean(&slopes);
    let avg_intercept = mean(&intercepts);

    let y1 = height;
    let y2 = height / 2;

    let x1 = ((y1 as f64 - avg_intercept) / avg_slope) as i32;
    let x2 = ((y2 as f64 - avg_intercept) / avg_slope) as i32;

    Ok(Line {
        slope: avg_slope,
        intercept: avg_intercept,
        x1,
        y1,
        x2,
        y2,
    })
}

fn draw_lane(image: &mut Mat, lane: &Line) -> Result<(), LineDetectionError> {
    line(
        image,
        Point::new(lane.x1, lane.y1),
        Point::new(lane.x2, lane.y2),
        Scalar::new(0.0, 0.0, 255.0, 0.0),
        10,
        LINE_AA,
        0,
    )?;
    Ok(())
}