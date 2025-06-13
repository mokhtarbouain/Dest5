```rust
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

pub fn filter_colors(input_image: &RgbaImage, is_day: bool) -> RgbaImage {
    let (width, height) = input_image.dimensions();
    let mut output_image = RgbaImage::new(width, height);

    let white_mask = create_color_mask(input_image, [255, 255, 255], is_day);
    let yellow_mask = create_color_mask(input_image, [255, 255, 0], is_day);
    let gray_mask = create_color_mask(input_image, [128, 128, 128], is_day);

    for (x, y, pixel) in input_image.enumerate_pixels() {
        let white = white_mask.get_pixel(x, y);
        let yellow = yellow_mask.get_pixel(x, y);
        let gray = gray_mask.get_pixel(x, y);

        let blended_pixel = blend_pixels(pixel, white, yellow, gray);
        output_image.put_pixel(x, y, blended_pixel);
    }

    output_image
}

fn create_color_mask(image: &RgbaImage, color: [u8; 3], is_day: bool) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut mask = RgbaImage::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let distance = color_distance(pixel, color);
        let threshold = if is_day { 50.0 } else { 100.0 };

        if distance < threshold {
            mask.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        } else {
            mask.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    mask
}

fn color_distance(pixel: &Rgba<u8>, color: [u8; 3]) -> f64 {
    let r_diff = (pixel[0] as i32 - color[0] as i32).pow(2);
    let g_diff = (pixel[1] as i32 - color[1] as i32).pow(2);
    let b_diff = (pixel[2] as i32 - color[2] as i32).pow(2);

    ((r_diff + g_diff + b_diff) as f64).sqrt()
}

fn blend_pixels(pixel: &Rgba<u8>, white: &Rgba<u8>, yellow: &Rgba<u8>, gray: &Rgba<u8>) -> Rgba<u8> {
    let mut blended = *pixel;

    if white[3] > 0 {
        blended = blend(blended, *white);
    }
    if yellow[3] > 0 {
        blended = blend(blended, *yellow);
    }
    if gray[3] > 0 {
        blended = blend(blended, *gray);
    }

    blended
}

fn blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    let alpha = overlay[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;

    let r = (base[0] as f32 * inv_alpha + overlay[0] as f32 * alpha) as u8;
    let g = (base[1] as f32 * inv_alpha + overlay[1] as f32 * alpha) as u8;
    let b = (base[2] as f32 * inv_alpha + overlay[2] as f32 * alpha) as u8;

    Rgba([r, g, b, 255])
}
```