```rust
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use imageproc::geometric_transformations::rotate;
use imageproc::filter::gaussian_blur_f32;
use imageproc::edges::canny;
use imageproc::rect::Rect;

pub fn grayscale(image: &RgbImage) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    image.to_luma8()
}

pub fn gaussian_blur(image: &ImageBuffer<image::Luma<u8>, Vec<u8>>, sigma: f32) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    gaussian_blur_f32(image, sigma)
}

pub fn edge_detection(image: &ImageBuffer<image::Luma<u8>, Vec<u8>>, low_threshold: f32, high_threshold: f32) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    canny(image, low_threshold, high_threshold)
}

pub fn roi_masking(image: &ImageBuffer<image::Luma<u8>, Vec<u8>>, rect: Rect) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let mut masked_image = ImageBuffer::new(image.width(), image.height());
    for (x, y, pixel) in image.enumerate_pixels() {
        if rect.contains(x, y) {
            masked_image.put_pixel(x, y, *pixel);
        }
    }
    masked_image
}
```