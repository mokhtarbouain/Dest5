use opencv::{
    core::{Mat, Point, Scalar, Size, CV_8UC1},
    imgproc::{fill_poly, get_structuring_element, morphology_ex, MorphShapes, MorphTypes},
    prelude::*,
};

/// Creates a trapezoid mask for the region of interest
pub fn create_trapezoid_mask(image: &Mat) -> Result<Mat, opencv::Error> {
    let rows = image.rows();
    let cols = image.cols();
    
    // Create an empty mask
    let mut mask = Mat::zeros(image.size()?, CV_8UC1)?.to_mat()?;
    
    // Define the large trapezoid (covers the bottom part of the image)
    let large_trapezoid_points = vec![
        Point::new(0, rows),                          // Bottom left
        Point::new(cols, rows),                       // Bottom right
        Point::new((cols * 5) / 6, rows / 2),         // Top right
        Point::new(cols / 6, rows / 2),               // Top left
    ];
    
    // Define the small trapezoid (covers the center part of the image)
    let small_trapezoid_points = vec![
        Point::new(cols / 3, rows),                   // Bottom left
        Point::new((cols * 2) / 3, rows),             // Bottom right
        Point::new((cols * 3) / 5, (rows * 2) / 3),   // Top right
        Point::new((cols * 2) / 5, (rows * 2) / 3),   // Top left
    ];
    
    // Create a vector of points arrays for fill_poly
    let large_trapezoid = vec![large_trapezoid_points];
    let small_trapezoid = vec![small_trapezoid_points];
    
    // Fill the trapezoids with white color (255)
    fill_poly(&mut mask, &large_trapezoid, Scalar::new(255.0, 0.0, 0.0, 0.0), opencv::imgproc::LINE_8, 0, Point::new(0, 0))?;
    fill_poly(&mut mask, &small_trapezoid, Scalar::new(255.0, 0.0, 0.0, 0.0), opencv::imgproc::LINE_8, 0, Point::new(0, 0))?;
    
    // Apply morphological operations to smooth the mask
    let kernel_size = Size::new(5, 5);
    let kernel = get_structuring_element(MorphShapes::MORPH_ELLIPSE as i32, kernel_size, Point::new(-1, -1))?;
    
    let mut processed_mask = Mat::default();
    morphology_ex(
        &mask,
        &mut processed_mask,
        MorphTypes::MORPH_CLOSE as i32,
        &kernel,
        Point::new(-1, -1),
        1,
        opencv::core::BORDER_CONSTANT,
        opencv::core::morphology_default_border_value()?,
    )?;
    
    Ok(processed_mask)
}

/// Applies the region of interest mask to the edge-detected image
pub fn apply_roi_mask(edge_image: &Mat) -> Result<Mat, opencv::Error> {
    let mask = create_trapezoid_mask(edge_image)?;
    
    let mut masked_image = Mat::default();
    opencv::core::bitwise_and(edge_image, &mask, &mut masked_image, &Mat::default())?;
    
    Ok(masked_image)
}