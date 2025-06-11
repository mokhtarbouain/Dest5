use anyhow::Result;
use opencv::{
    core::Mat,
    imgproc::{canny, cvt_color, gaussian_blur, COLOR_BGR2GRAY},
    prelude::*,
};

const GAUSSIAN_KERNEL_SIZE: i32 = 5;
const CANNY_LOW_THRESHOLD: f64 = 50.0;
const CANNY_HIGH_THRESHOLD: f64 = 150.0;

pub fn apply_grayscale(image: &Mat) -> Result<Mat> {
    let mut gray_image = Mat::default();
    cvt_color(image, &mut gray_image, COLOR_BGR2GRAY, 0)?;
    Ok(gray_image)
}

pub fn apply_gaussian_blur(image: &Mat) -> Result<Mat> {
    let mut blurred_image = Mat::default();
    let kernel_size = opencv::core::Size::new(GAUSSIAN_KERNEL_SIZE, GAUSSIAN_KERNEL_SIZE);
    gaussian_blur(
        image,
        &mut blurred_image,
        kernel_size,
        0.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
    )?;
    Ok(blurred_image)
}

pub fn apply_canny(image: &Mat) -> Result<Mat> {
    let mut edges = Mat::default();
    canny(
        image,
        &mut edges,
        CANNY_LOW_THRESHOLD,
        CANNY_HIGH_THRESHOLD,
        3,
        false,
    )?;
    Ok(edges)
}