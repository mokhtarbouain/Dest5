```rust
use opencv::prelude::*;
use opencv::imgcodecs;
use opencv::imgproc;
use std::error::Error;

pub fn apply_grayscale(input_image_path: &str, output_image_path: &str) -> Result<(), Box<dyn Error>> {
    let input_image = imgcodecs::imread(input_image_path, imgcodecs::IMREAD_COLOR)?;
    if input_image.size()?.width == 0 || input_image.size()?.height == 0 {
        return Err("Failed to read input image".into());
    }

    let mut grayscale_image = Mat::default()?;

    imgproc::cvt_color(&input_image, &mut grayscale_image, imgproc::COLOR_BGR2GRAY, 0)?;

    let success = imgcodecs::imwrite(output_image_path, &grayscale_image, &opencv::types::VectorOfi32::new())?;
    if !success {
        return Err("Failed to write grayscale image".into());
    }

    Ok(())
}
```