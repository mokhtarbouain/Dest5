```rust
use opencv::core::{Mat, Size};
use opencv::imgcodecs;
use opencv::imgproc;
use std::fs;
use std::path::Path;

pub fn apply_gaussian_blur(input_image_path: &str, output_image_path: &str, kernel_size: (i32, i32), sigma_x: f64, sigma_y: f64) -> Result<(), String> {
    if !Path::new(input_image_path).exists() {
        return Err(format!("Input image path does not exist: {}", input_image_path));
    }

    let input_image = imgcodecs::imread(input_image_path, imgcodecs::IMREAD_COLOR)
        .map_err(|e| format!("Failed to read input image: {}", e))?;

    if input_image.size().width == 0 || input_image.size().height == 0 {
        return Err("Input image is invalid or empty".to_string());
    }

    let mut blurred_image = Mat::default()
        .map_err(|e| format!("Failed to create default Mat: {}", e))?;

    imgproc::GaussianBlur(
        &input_image,
        &mut blurred_image,
        Size::new(kernel_size.0, kernel_size.1),
        sigma_x,
        sigma_y,
        imgproc::BORDER_DEFAULT,
    ).map_err(|e| format!("Failed to apply Gaussian blur: {}", e))?;

    if let Some(parent) = Path::new(output_image_path).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    imgcodecs::imwrite(output_image_path, &blurred_image, &imgcodecs::IMWRITE_PNG_COMPRESSION)
        .map_err(|e| format!("Failed to write output image: {}", e))?;

    Ok(())
}
```