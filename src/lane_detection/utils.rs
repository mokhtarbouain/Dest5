use opencv::{
    core::{Point, Scalar, Vec3b, Vec4f},
    imgproc::{fill_poly, line},
    prelude::*,
};

/// Calculates the slope between two points
pub fn calculate_slope(p1: Point, p2: Point) -> f64 {
    if p2.x - p1.x == 0 {
        return f64::MAX; // Vertical line
    }
    (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64
}

/// Calculates the y-intercept given slope and a point
pub fn calculate_intercept(slope: f64, point: Point) -> f64 {
    point.y as f64 - slope * point.x as f64
}

/// Draws blue lines for left and right lanes on the frame
pub fn draw_lane_lines(frame: &mut Mat, left_lane: &[Point], right_lane: &[Point]) -> Result<(), opencv::Error> {
    let blue_color = Scalar::new(255.0, 0.0, 0.0, 0.0); // BGR format
    let line_thickness = 7;

    // Draw left lane line
    for i in 0..left_lane.len().saturating_sub(1) {
        line(
            frame,
            left_lane[i],
            left_lane[i + 1],
            blue_color,
            line_thickness,
            opencv::imgproc::LINE_AA,
            0,
        )?;
    }

    // Draw right lane line
    for i in 0..right_lane.len().saturating_sub(1) {
        line(
            frame,
            right_lane[i],
            right_lane[i + 1],
            blue_color,
            line_thickness,
            opencv::imgproc::LINE_AA,
            0,
        )?;
    }

    Ok(())
}

/// Highlights the lane area with light blue color
pub fn highlight_lane_area(
    frame: &mut Mat,
    left_lane: &[Point],
    right_lane: &[Point],
) -> Result<(), opencv::Error> {
    let mut lane_mask = Mat::zeros(frame.size()?, opencv::core::CV_8UC3)?.to_mat()?;
    
    // Create polygon points by combining left and right lane points
    let mut polygon_points = Vec::new();
    
    // Add left lane points in forward order
    polygon_points.extend_from_slice(left_lane);
    
    // Add right lane points in reverse order to form a closed polygon
    for point in right_lane.iter().rev() {
        polygon_points.push(*point);
    }
    
    if !polygon_points.is_empty() {
        // Convert points to a format suitable for fill_poly
        let points_array = vec![polygon_points];
        
        // Light blue color (BGR format)
        let light_blue = Scalar::new(52.0, 229.0, 235.0, 0.0);
        
        // Fill the polygon
        fill_poly(&mut lane_mask, &points_array, light_blue, opencv::imgproc::LINE_AA, 0, Point::new(0, 0))?;
        
        // Blend the mask with the original frame
        // Original frame weight: 90%, mask weight: 30%
        opencv::core::add_weighted(frame, 0.9, &lane_mask, 0.3, 0.0, frame, -1)?;
    }
    
    Ok(())
}