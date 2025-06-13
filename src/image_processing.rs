```rust
use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};

pub fn load_image(file_path: &str) -> Result<image::DynamicImage, image::ImageError> {
    image::open(file_path)
}

pub fn display_image(image: &image::DynamicImage) {
    let width = image.width() as usize;
    let height = image.height() as usize;

    let mut window = Window::new(
        "Image Viewer",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer: Vec<u32> = image
            .pixels()
            .map(|(_, _, pixel)| {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let a = pixel[3];
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect();

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}
```