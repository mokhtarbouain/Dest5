

use image::{GenericImageView, RgbaImage};
use imageproc::drawing::{draw_line_segment_mut, draw_rectangle_mut};
use imageproc::rect::Rect;
use num::abs;

pub struct RegionOfInterest {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl RegionOfInterest {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        RegionOfInterest { x, y, width, height }
    }

    pub fn create_mask(&self, edges: &RgbaImage) -> RgbaImage {
        let mut mask = RgbaImage::new(edges.width(), edges.height());
        draw_rectangle_mut(&mut mask, Rect::at(self.x, self.y).of_size(self.width, self.height), image::Rgba([255, 255, 255, 255]));
        let mut masked_edges = edges.clone();
        for x in 0..edges.width() {
            for y in 0..edges.height() {
                if mask.get_pixel(x, y).0[3] == 0 {
                    masked_edges.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
                }
            }
        }
        masked_edges
    }
}