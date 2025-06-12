use opencv::{
    core,
    imgproc,
    prelude::*,
};

/// Converts a color image to grayscale
pub fn convert_to_grayscale(image: &Mat) -> Result<Mat, opencv::Error> {
    let mut gray = Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(gray)
}

/// Applies Gaussian blur to an image
pub fn apply_gaussian_blur(image: &Mat) -> Result<Mat, opencv::Error> {
    let mut blurred = Mat::default();
    let kernel_size = core::Size::new(3, 3);
    imgproc::gaussian_blur(&image, &mut blurred, kernel_size, 0.0, 0.0, core::BORDER_DEFAULT)?;
    Ok(blurred)
}

/// Applies Canny edge detection to an image
pub fn detect_edges(image: &Mat) -> Result<Mat, opencv::Error> {
    let mut edges = Mat::default();
    imgproc::canny(&image, &mut edges, 50.0, 150.0, 3, false)?;
    Ok(edges)
}

/// Processes an image through grayscale conversion, Gaussian blur, and edge detection
pub fn process_edges(image: &Mat) -> Result<Mat, opencv::Error> {
    let gray = convert_to_grayscale(&image)?;
    let blurred = apply_gaussian_blur(&gray)?;
    let edges = detect_edges(&blurred)?;
    Ok(edges)
}