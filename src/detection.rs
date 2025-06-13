```rust
use image::{GenericImageView, ImageBuffer, RgbImage};
use imageproc::edges::canny;
use imageproc::hough::hough_lines;
use imageproc::rect::Rect;
use imageproc::drawing::draw_line_segment_mut;

pub fn detect_edges(image: &RgbImage) -> RgbImage {
    let edges = canny(image, 10.0, 20.0);
    edges
}

pub fn detect_lanes(image: &RgbImage) -> RgbImage {
    let edges = canny(image, 10.0, 20.0);
    let lines = hough_lines(&edges, 1.0, std::f64::consts::PI / 180.0, 100);

    let mut output_image = image.clone();
    for line in lines {
        draw_line_segment_mut(&mut output_image, (line[0] as i32, line[1] as i32), (line[2] as i32, line[3] as i32), imageproc::drawing::Rgb([255, 0, 0]));
    }

    output_image
}
```