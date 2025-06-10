

use image::{GenericImageView, RgbaImage};
use imageproc::drawing::{draw_line_segment_mut, draw_polygon_mut};
use imageproc::gradients::non_max_suppression;
use imageproc::gradients::{grad, grad_x, grad_y};
use imageproc::gradients::{sobel, sobel_x, sobel_y};
use imageproc::morphology::{erode, dilate};
use imageproc::rect::Rect;
use imageproc::stats::histogram;
use imageproc::stats::mean;
use num::complex::Complex;
use num::Float;
use std::f64::consts::PI;

fn detect_lines(edge_map: &RgbaImage) -> Vec<(f64, f64, f64, f64)> {
    let (width, height) = edge_map.dimensions();
    let mut lines = Vec::new();
    let mut theta = 0.0;
    let mut rho = 0.0;
    let mut max_rho = 0.0;
    let mut max_theta = 0.0;
    let mut max_votes = 0;
    let mut votes = 0;
    let mut x1 = 0.0;
    let mut y1 = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;

    for y in 0..height {
        for x in 0..width {
            let pixel = edge_map.get_pixel(x, y);
            if pixel.0[0] > 0 {
                for theta in 0..180 {
                    rho = (x as f64 * theta as f64 + y as f64 * theta as f64).atan2((x as f64 - y as f64) as f64);
                    if rho > max_rho {
                        max_rho = rho;
                        max_theta = theta as f64;
                        max_votes = votes;
                    }
                    votes += 1;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let pixel = edge_map.get_pixel(x, y);
            if pixel.0[0] > 0 {
                x1 = x as f64;
                y1 = y as f64;
                x2 = x as f64 + max_rho * max_theta as f64;
                y2 = y as f64 + max_rho * max_theta as f64;
                lines.push((x1, y1, x2, y2));
            }
        }
    }

    lines
}

fn draw_lanes(image: &mut RgbaImage, lines: Vec<(f64, f64, f64, f64)>, color: image::Rgba<u8>) {
    for line in lines {
        draw_line_segment_mut(image, (line.0 as u32, line.1 as u32), (line.2 as u32, line.3 as u32), color);
    }
}

fn houghLines(edge_map: &RgbaImage) -> Vec<(f64, f64, f64, f64)> {
    let (width, height) = edge_map.dimensions();
    let mut lines = Vec::new();
    let mut theta = 0.0;
    let mut rho = 0.0;
    let mut max_rho = 0.0;
    let mut max_theta = 0.0;
    let mut max_votes = 0;
    let mut votes = 0;
    let mut x1 = 0.0;
    let mut y1 = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;

    for y in 0..height {
        for x in 0..width {
            let pixel = edge_map.get_pixel(x, y);
            if pixel.0[0] > 0 {
                for theta in 0..180 {
                    rho = (x as f64 * theta as f64 + y as f64 * theta as f64).atan2((x as f64 - y as f64) as f64);
                    if rho > max_rho {
                        max_rho = rho;
                        max_theta = theta as f64;
                        max_votes = votes;
                    }
                    votes += 1;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let pixel = edge_map.get_pixel(x, y);
            if pixel.0[0] > 0 {
                x1 = x as f64;
                y1 = y as f64;
                x2 = x as f64 + max_rho * max_theta as f64;
                y2 = y as f64 + max_rho * max_theta as f64;
                lines.push((x1, y1, x2, y2));
            }
        }
    }

    lines
}