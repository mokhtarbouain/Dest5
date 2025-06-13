```rust
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::drawing::draw_filled_polygon_mut;
use nalgebra::{Point2, Vector2};

pub fn region_of_interest(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, top_left: Point2<f32>, top_right: Point2<f32>, bottom_left: Point2<f32>, bottom_right: Point2<f32>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut mask = ImageBuffer::from_pixel(image.width(), image.height(), Rgba([0, 0, 0, 0]));

    let trapezoid_points = vec![
        Point2::new(top_left.x, top_left.y),
        Point2::new(top_right.x, top_right.y),
        Point2::new(bottom_right.x, bottom_right.y),
        Point2::new(bottom_left.x, bottom_left.y),
    ];

    draw_filled_polygon_mut(&mut mask, &trapezoid_points, Rgba([255, 255, 255, 255]));

    let mut output_image = ImageBuffer::from_pixel(image.width(), image.height(), Rgba([0, 0, 0, 0]));

    for (x, y, pixel) in image.enumerate_pixels() {
        let mask_pixel = mask.get_pixel(x, y);
        if mask_pixel[0] > 0 {
            output_image.put_pixel(x, y, *pixel);
        }
    }

    output_image
}
```