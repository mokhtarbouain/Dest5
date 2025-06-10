use clap::{App, Arg};
use env_logger;
use log::{error, info};
use opencv::{
    core::{Mat, Point, Scalar, Size, Vec3b, CV_8UC3},
    highgui::{imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        cvt_color, gaussian_blur, canny, hough_lines_p, line, 
        COLOR_BGR2GRAY, COLOR_BGR2HSV, THRESH_BINARY
    },
    prelude::*,
    types::VectorOfVec4i,
};
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = App::new("Lane Detection")
        .version("1.0")
        .author("Lane Detection Team")
        .about("Detects lanes in road images")
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("FILE")
                .help("Sets the input image file")
                .required(true),
        )
        .get_matches();

    let image_path = matches.value_of("image").unwrap();
    info!("Processing image: {}", image_path);

    let image = imread(image_path, IMREAD_COLOR)?;
    if image.empty() {
        error!("Could not read image: {}", image_path);
        return Err("Failed to load image".into());
    }

    let result = process_image(&image)?;

    imshow("Lane Detection Result", &result)?;
    wait_key(0)?;

    Ok(())
}

fn process_image(image: &Mat) -> Result<Mat, Box<dyn Error>> {
    let mut result = image.clone();
    
    // Determine if image is daytime
    let is_daytime = detect_daytime(image)?;
    info!("Image is taken during {}", if is_daytime { "daytime" } else { "nighttime" });

    // Apply region of interest mask
    let roi_mask = create_roi_mask(image)?;
    
    // Filter colors based on time of day
    let filtered = filter_colors(image, is_daytime)?;
    
    // Convert to grayscale
    let mut gray = Mat::default();
    cvt_color(&filtered, &mut gray, COLOR_BGR2GRAY, 0)?;
    
    // Apply Gaussian blur
    let mut blurred = Mat::default();
    gaussian_blur(&gray, &mut blurred, Size::new(5, 5), 0.0, 0.0, opencv::core::BORDER_DEFAULT)?;
    
    // Detect edges
    let mut edges = Mat::default();
    canny(&blurred, &mut edges, 50.0, 150.0, 3, false)?;
    
    // Apply ROI mask to edge map
    let mut masked_edges = Mat::default();
    opencv::core::bitwise_and(&edges, &roi_mask, &mut masked_edges, &Mat::default())?;
    
    // Detect lines using Hough transform
    let mut lines = VectorOfVec4i::new();
    hough_lines_p(
        &masked_edges,
        &mut lines,
        1.0,
        std::f64::consts::PI / 180.0,
        50,
        50.0,
        10.0,
    )?;
    
    // Draw lanes on the original image
    draw_lanes(&mut result, &lines)?;
    
    Ok(result)
}

fn detect_daytime(image: &Mat) -> Result<bool, Box<dyn Error>> {
    let mut hsv = Mat::default();
    cvt_color(image, &mut hsv, COLOR_BGR2HSV, 0)?;
    
    let mut brightness_sum = 0.0;
    let total_pixels = (image.rows() * image.cols()) as f64;
    
    for row in 0..hsv.rows() {
        for col in 0..hsv.cols() {
            let pixel = hsv.at_2d::<Vec3b>(row, col)?;
            brightness_sum += pixel[2] as f64;  // V channel in HSV
        }
    }
    
    let avg_brightness = brightness_sum / total_pixels;
    Ok(avg_brightness > 100.0)  // Threshold for daytime
}

fn create_roi_mask(image: &Mat) -> Result<Mat, Box<dyn Error>> {
    let rows = image.rows();
    let cols = image.cols();
    
    let mut mask = Mat::new_rows_cols(rows, cols, CV_8UC3)?;
    mask.set_scalar(Scalar::all(0.0))?;
    
    let roi_points = vec![
        Point::new(0, rows),
        Point::new(cols / 3, rows / 2),
        Point::new(cols * 2 / 3, rows / 2),
        Point::new(cols, rows),
    ];
    
    let points_vec = opencv::types::VectorOfPoint::from_iter(roi_points);
    let mut roi_contours = opencv::types::VectorOfVectorOfPoint::new();
    roi_contours.push(points_vec);
    
    opencv::imgproc::fill_poly(
        &mut mask,
        &roi_contours,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        opencv::imgproc::LINE_8,
        0,
        Point::new(0, 0),
    )?;
    
    Ok(mask)
}

fn filter_colors(image: &Mat, is_daytime: bool) -> Result<Mat, Box<dyn Error>> {
    let mut hsv = Mat::default();
    cvt_color(image, &mut hsv, COLOR_BGR2HSV, 0)?;
    
    let mut yellow_mask = Mat::default();
    let mut white_mask = Mat::default();
    
    if is_daytime {
        // Yellow color range
        opencv::core::in_range(
            &hsv,
            &Scalar::new(20.0, 100.0, 100.0, 0.0),
            &Scalar::new(30.0, 255.0, 255.0, 0.0),
            &mut yellow_mask,
        )?;
        
        // White color range
        opencv::core::in_range(
            &hsv,
            &Scalar::new(0.0, 0.0, 200.0, 0.0),
            &Scalar::new(180.0, 30.0, 255.0, 0.0),
            &mut white_mask,
        )?;
    } else {
        // Nighttime: adjust thresholds for yellow
        opencv::core::in_range(
            &hsv,
            &Scalar::new(20.0, 80.0, 80.0, 0.0),
            &Scalar::new(30.0, 255.0, 255.0, 0.0),
            &mut yellow_mask,
        )?;
        
        // Nighttime: adjust thresholds for white
        opencv::core::in_range(
            &hsv,
            &Scalar::new(0.0, 0.0, 160.0, 0.0),
            &Scalar::new(180.0, 40.0, 255.0, 0.0),
            &mut white_mask,
        )?;
    }
    
    // Combine masks
    let mut color_mask = Mat::default();
    opencv::core::bitwise_or(&yellow_mask, &white_mask, &mut color_mask, &Mat::default())?;
    
    // Apply mask to original image
    let mut filtered = Mat::default();
    opencv::core::bitwise_and(image, image, &mut filtered, &color_mask)?;
    
    Ok(filtered)
}

fn draw_lanes(image: &mut Mat, lines: &VectorOfVec4i) -> Result<(), Box<dyn Error>> {
    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();
    
    // Separate lines into left and right based on slope
    for i in 0..lines.len() {
        let line = lines.get(i)?;
        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];
        
        if x2 == x1 {
            continue;  // Skip vertical lines
        }
        
        let slope = (y2 - y1) as f64 / (x2 - x1) as f64;
        
        // Filter out lines with small slope
        if slope.abs() < 0.5 {
            continue;
        }
        
        if slope < 0.0 {
            left_lines.push((x1, y1, x2, y2, slope));
        } else {
            right_lines.push((x1, y1, x2, y2, slope));
        }
    }
    
    // Calculate average left and right lanes
    if let Some((x1, y1, x2, y2)) = average_lane(&left_lines) {
        line(
            image,
            Point::new(x1, y1),
            Point::new(x2, y2),
            Scalar::new(0.0, 0.0, 255.0, 0.0),  // Red
            5,
            opencv::imgproc::LINE_AA,
            0,
        )?;
    }
    
    if let Some((x1, y1, x2, y2)) = average_lane(&right_lines) {
        line(
            image,
            Point::new(x1, y1),
            Point::new(x2, y2),
            Scalar::new(0.0, 0.0, 255.0, 0.0),  // Red
            5,
            opencv::imgproc::LINE_AA,
            0,
        )?;
    }
    
    Ok(())
}

fn average_lane(lines: &[(i32, i32, i32, i32, f64)]) -> Option<(i32, i32, i32, i32)> {
    if lines.is_empty() {
        return None;
    }
    
    let mut sum_x1 = 0;
    let mut sum_y1 = 0;
    let mut sum_x2 = 0;
    let mut sum_y2 = 0;
    
    for &(x1, y1, x2, y2, _) in lines {
        sum_x1 += x1;
        sum_y1 += y1;
        sum_x2 += x2;
        sum_y2 += y2;
    }
    
    let n = lines.len() as i32;
    let avg_x1 = sum_x1 / n;
    let avg_y1 = sum_y1 / n;
    let avg_x2 = sum_x2 / n;
    let avg_y2 = sum_y2 / n;
    
    Some((avg_x1, avg_y1, avg_x2, avg_y2))
}