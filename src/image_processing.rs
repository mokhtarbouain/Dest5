

use image::{GenericImageView, RgbaImage, Rgba};
use imageproc::gradients::{gradients, Gradients, Sobel};
use imageproc::gradients::SobelOperator;
use imageproc::gradients::SobelVariant;
use imageproc::gradients::SobelDerivative;
use imageproc::gradients::SobelType;
use imageproc::gradients::SobelOperatorType;
use imageproc::drawing::{draw_line_segment_mut, draw_rectangle_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;
use std::io::Read;

pub fn load_image(path: &str) -> RgbaImage {
    image::open(path).unwrap().into_rgba8()
}

pub fn apply_roi_mask(image: &mut RgbaImage, mask: &RgbaImage) {
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if mask.get_pixel(x, y).0[3] == 0 {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

pub fn filter_colors(image: &mut RgbaImage, min: u8, max: u8) {
    for pixel in image.pixels_mut() {
        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];
        if r < min || r > max || g < min || g > max || b < min || b > max {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

pub fn apply_grayscale(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;
        let gray = (0.2126 * r + 0.7152 * g + 0.0722 * b) as u8;
        *pixel = Rgba([gray, gray, gray, pixel.0[3]]);
    }
}

pub fn apply_gaussian_blur(image: &mut RgbaImage, sigma: f64) {
    let kernel = imageproc::gradients::gaussian_blur_kernel(sigma);
    imageproc::gradients::gaussian_blur(image, &kernel);
}

pub fn detect_edges(image: &mut RgbaImage) {
    let gradients = gradients(image, SobelOperator::new(SobelVariant::Scharr));
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let gradient = gradients.get_pixel(x, y);
        let magnitude = (gradient.0[0] as f64).powi(2) + (gradient.0[1] as f64).powi(2);
        let magnitude = (magnitude as f64).sqrt() as u8;
        *pixel = Rgba([magnitude, magnitude, magnitude, pixel.0[3]]);
    }
}

pub fn display_image(image: &RgbaImage) {
    let (width, height) = image.dimensions();
    let mut buffer = Vec::with_capacity(width as usize * height as usize * 4);
    for pixel in image.pixels() {
        buffer.extend_from_slice(&pixel.0);
    }
    let mut file = fs::File::create("image.png").unwrap();
    let mut encoder = png::Encoder::new(file);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let (info, mut writer) = encoder.write_header().unwrap();
    let mut data = Vec::with_capacity(info.buffer_size());
    writer.write_image_data(&buffer).unwrap();
}

pub fn apply_canny(image: &mut RgbaImage, low_threshold: u8, high_threshold: u8) {
    let gradients = gradients(image, SobelOperator::new(SobelVariant::Scharr));
    let mut non_max_suppressed = RgbaImage::new(image.width(), image.height());
    imageproc::gradients::non_max_suppression(&gradients, &mut non_max_suppressed);
    let mut double_thresholded = RgbaImage::new(image.width(), image.height());
    imageproc::gradients::double_threshold(&non_max_suppressed, low_threshold, high_threshold, &mut double_thresholded);
    let mut edges = RgbaImage::new(image.width(), image.height());
    imageproc::gradients::hysteresis(&double_thresholded, &mut edges);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if edges.get_pixel(x, y).0[3] == 0 {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

pub fn region_of_interest(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32) {
    let mut mask = RgbaImage::new(image.width(), image.height());
    draw_rectangle_mut(&mut mask, Rect::at(x, y).of_size(width, height), Rgba([255, 255, 255, 255]));
    apply_roi_mask(image, &mask);
}