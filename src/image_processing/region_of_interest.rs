use anyhow::Result;
use opencv::{
    core::{bitwise_and, Mat, Point, Scalar, VecN},
    imgproc::{fill_poly, LINE_8},
    prelude::*,
};

pub fn region_of_interest(image: &Mat) -> Result<Mat> {
    let mut mask = Mat::zeros(image.size()?, image.typ()?)?;
    
    let height = image.rows() as f32;
    let width = image.cols() as f32;
    
    // Define the polygon vertices as percentages of image dimensions
    let bottom_left = Point::new((width * 0.1) as i32, height as i32);
    let bottom_right = Point::new((width * 0.9) as i32, height as i32);
    let top_right = Point::new((width * 0.6) as i32, (height * 0.6) as i32);
    let top_left = Point::new((width * 0.4) as i32, (height * 0.6) as i32);
    
    // Create polygon points array
    let pts: Vec<Point> = vec![bottom_left, bottom_right, top_right, top_left];
    let pts_array = vec![pts];
    
    // Fill the polygon on the mask
    fill_poly(
        &mut mask,
        &pts_array,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        LINE_8,
        0,
        Point::new(0, 0),
    )?;
    
    // Apply the mask to the image
    let mut masked_image = Mat::default();
    bitwise_and(image, &mask, &mut masked_image, &Mat::default())?;
    
    Ok(masked_image)
}