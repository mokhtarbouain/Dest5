```rust
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use nalgebra::{DMatrix, DVector};
use std::f64;

pub fn draw_lanes(image: &mut RgbaImage, lines: Vec<(f64, f64, f64, f64)>) {
    let (left_lane, right_lane) = categorize_lines(lines);

    if let Some(left_lane) = left_lane {
        let left_fit = linear_regression(&left_lane);
        draw_lane(image, &left_fit, 0);
    }

    if let Some(right_lane) = right_lane {
        let right_fit = linear_regression(&right_lane);
        draw_lane(image, &right_fit, 1);
    }
}

fn categorize_lines(lines: Vec<(f64, f64, f64, f64)>) -> (Option<Vec<(f64, f64)>>, Option<Vec<(f64, f64)>>) {
    let mut left_lane = Vec::new();
    let mut right_lane = Vec::new();

    for (x1, y1, x2, y2) in lines {
        let slope = (y2 - y1) / (x2 - x1);
        if slope < 0.0 {
            left_lane.push((x1, y1));
            left_lane.push((x2, y2));
        } else {
            right_lane.push((x1, y1));
            right_lane.push((x2, y2));
        }
    }

    let left_lane = if left_lane.is_empty() { None } else { Some(left_lane) };
    let right_lane = if right_lane.is_empty() { None } else { Some(right_lane) };

    (left_lane, right_lane)
}

fn linear_regression(points: &[(f64, f64)]) -> (f64, f64) {
    let n = points.len() as f64;
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut xy_sum = 0.0;
    let mut xx_sum = 0.0;

    for &(x, y) in points {
        x_sum += x;
        y_sum += y;
        xy_sum += x * y;
        xx_sum += x * x;
    }

    let slope = (n * xy_sum - x_sum * y_sum) / (n * xx_sum - x_sum * x_sum);
    let intercept = (y_sum - slope * x_sum) / n;

    (slope, intercept)
}

fn draw_lane(image: &mut RgbaImage, fit: &(f64, f64), lane_type: u8) {
    let (slope, intercept) = *fit;
    let height = image.height();
    let width = image.width();

    let mut y1 = height as f64;
    let mut y2 = height as f64 * 0.6;

    let x1 = (y1 - intercept) / slope;
    let x2 = (y2 - intercept) / slope;

    let color = if lane_type == 0 { Rgba([0, 255, 0, 255]) } else { Rgba([255, 0, 0, 255]) };

    image.draw_line(
        (x1 as u32, y1 as u32),
        (x2 as u32, y2 as u32),
        color,
    );
}
```