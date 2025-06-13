```rust
use image::{DynamicImage, GenericImageView};

pub fn is_daytime(image: &DynamicImage) -> bool {
    let (width, height) = image.dimensions();
    let mut sum_r = 0u32;
    let mut sum_g = 0u32;
    let mut sum_b = 0u32;

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            sum_r += pixel[0] as u32;
            sum_g += pixel[1] as u32;
            sum_b += pixel[2] as u32;
        }
    }

    let mean_r = sum_r / (width * height) as u32;
    let mean_g = sum_g / (width * height) as u32;
    let mean_b = sum_b / (width * height) as u32;

    mean_r > 100 && mean_g > 100 && mean_b > 100
}
```