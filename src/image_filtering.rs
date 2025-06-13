```rust
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::path::Path;

pub fn filter_colors(input_path: &Path, output_path: &Path, is_day: bool) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    let mut img = img.to_rgba8();

    let (mask_color, blend_color) = if is_day {
        (Rgba([255, 255, 255, 255]), Rgba([255, 255, 0, 255]))
    } else {
        (Rgba([0, 0, 0, 255]), Rgba([0, 0, 255, 255]))
    };

    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            if pixel == mask_color {
                img.put_pixel(x, y, blend_color);
            }
        }
    }

    img.save(output_path)?;
    Ok(())
}
```