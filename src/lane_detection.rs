```rust
use opencv::{
    core::{Mat, Point, Scalar, Size},
    imgcodecs,
    imgproc::{HoughLinesP, Canny, cvtColor, COLOR_BGR2GRAY, LINE_AA},
    prelude::*,
};
use std::error::Error;

pub fn hough_lines(
    image_path: &str,
    draw_lines: bool,
    rho: f64,
    theta: f64,
    threshold: i32,
    min_line_length: f64,
    max_line_gap: f64,
) -> Result<(), Box<dyn Error>> {
    if rho <= 0.0 || theta <= 0.0 || threshold <= 0 || min_line_length <= 0.0 || max_line_gap <= 0.0 {
        return Err("Invalid parameters".into());
    }

    let mut image = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;
    if image.size()? == Size::new(0, 0) {
        return Err("Failed to read image".into());
    }

    let mut gray_image = Mat::default()?;
    let mut edges = Mat::default()?;
    let mut lines = Mat::default()?;

    cvtColor(&image, &mut gray_image, COLOR_BGR2GRAY, 0)?;
    Canny(&gray_image, &mut edges, 50.0, 200.0, 3, false)?;

    HoughLinesP(
        &edges,
        &mut lines,
        rho,
        theta,
        threshold,
        min_line_length,
        max_line_gap,
    )?;

    if draw_lines {
        for i in 0..lines.rows() {
            let line = lines.row(i)?;
            let start = Point::new(line.at::<i32>(0)?, line.at::<i32>(1)?);
            let end = Point::new(line.at::<i32>(2)?, line.at::<i32>(3)?);
            opencv::imgproc::line(
                &mut image,
                start,
                end,
                Scalar::new(0.0, 0.0, 255.0, 0.0),
                2,
                LINE_AA,
                0,
            )?;
        }
    }

    imgcodecs::imwrite("output.jpg", &image, &opencv::types::VectorOfi32::new())?;
    Ok(())
}
```