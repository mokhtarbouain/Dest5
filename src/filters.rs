```rust
use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};

pub fn filter_white(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut filtered_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;
        if r > 200 && g > 200 && b > 200 {
            filtered_image.put_pixel(x, y, *pixel);
        } else {
            filtered_image.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    filtered_image
}

pub fn filter_yellow(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut filtered_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;
        if r > 200 && g > 200 && b < 100 {
            filtered_image.put_pixel(x, y, *pixel);
        } else {
            filtered_image.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    filtered_image
}

pub fn filter_gray(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut filtered_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;
        let gray = (r as u32 + g as u32 + b as u32) / 3;
        filtered_image.put_pixel(x, y, Rgb([gray as u8, gray as u8, gray as u8]));
    }

    filtered_image
}
```