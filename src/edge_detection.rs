```rust
use opencv::prelude::*;
use opencv::imgcodecs;
use opencv::imgproc;
use std::fs;
use std::path::Path;

pub fn apply_canny(input_image_path: &str, output_image_path: &str, low_threshold: f64, high_threshold: f64) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(input_image_path).exists() {
        return Err("Input image path does not exist".into());
    }

    if low_threshold < 0.0 || high_threshold < 0.0 || low_threshold > high_threshold {
        return Err("Threshold values are invalid".into());
    }

    let input_image = imgcodecs::imread(input_image_path, imgcodecs::IMREAD_GRAYSCALE)?;
    if input_image.size()?.width == 0 || input_image.size()?.height == 0 {
        return Err("Input image is empty".into());
    }

    let mut edges = Mat::default()?;

    imgproc::Canny(&input_image, &mut edges, low_threshold, high_threshold, 3, false, imgproc::BORDER_DEFAULT)?;

    if !imgcodecs::imwrite(output_image_path, &edges, &imgcodecs::IMWRITE_PNG_COMPRESSION, 3, &imgcodecs::IMWRITE_PNG_STRATEGY_DEFAULT, 0)? {
        return Err("Failed to write output image".into());
    }

    Ok(())
}
```