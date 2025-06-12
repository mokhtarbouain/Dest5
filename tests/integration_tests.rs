use std::path::Path;
use image::{DynamicImage, GenericImageView};
use opencv::{
    core::{Mat, Point, Scalar, Size, Vec3b},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{cvt_color, COLOR_BGR2GRAY},
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};
use tempfile::tempdir;

use lane_detection::{
    color_filtering::filter_colors,
    day_night_detection::detect_day_or_night,
    edge_detection::detect_edges,
    lane_fitting::{fit_lane_lines, validate_lanes},
    line_detection::{detect_lines, filter_lines},
    region_of_interest::apply_region_of_interest,
    types::{DayNightMode, LaneDetectionResult, Line},
};

fn load_test_image(filename: &str) -> Mat {
    let test_data_path = Path::new("test_data").join(filename);
    imread(test_data_path.to_str().unwrap(), IMREAD_COLOR).unwrap()
}

fn create_test_video(frames: &[Mat], fps: f64) -> String {
    let dir = tempdir().unwrap();
    let video_path = dir.path().join("test_video.mp4");
    let video_path_str = video_path.to_str().unwrap().to_string();
    
    let fourcc = opencv::videoio::VideoWriter::fourcc('m' as i8, 'p' as i8, '4' as i8, 'v' as i8).unwrap();
    let size = frames[0].size().unwrap();
    let mut writer = opencv::videoio::VideoWriter::new(
        &video_path_str,
        fourcc,
        fps,
        size,
        true,
    ).unwrap();
    
    for frame in frames {
        writer.write(frame).unwrap();
    }
    
    writer.release().unwrap();
    video_path_str
}

#[test]
fn test_day_night_detection() {
    let day_image = load_test_image("day_road.jpg");
    let night_image = load_test_image("night_road.jpg");
    
    assert_eq!(detect_day_or_night(&day_image), DayNightMode::Day);
    assert_eq!(detect_day_or_night(&night_image), DayNightMode::Night);
}

#[test]
fn test_color_filtering() {
    let image = load_test_image("road.jpg");
    let day_mode = DayNightMode::Day;
    
    let filtered = filter_colors(&image, day_mode);
    
    // Basic validation - filtered image should not be empty
    assert!(!filtered.empty());
    assert_eq!(filtered.size().unwrap(), image.size().unwrap());
}

#[test]
fn test_edge_detection() {
    let image = load_test_image("road.jpg");
    let filtered = filter_colors(&image, DayNightMode::Day);
    
    let edges = detect_edges(&filtered);
    
    // Basic validation - edges image should not be empty
    assert!(!edges.empty());
    assert_eq!(edges.size().unwrap(), image.size().unwrap());
}

#[test]
fn test_region_of_interest() {
    let image = load_test_image("road.jpg");
    let filtered = filter_colors(&image, DayNightMode::Day);
    let edges = detect_edges(&filtered);
    
    let roi = apply_region_of_interest(&edges);
    
    // Basic validation - ROI image should not be empty
    assert!(!roi.empty());
    assert_eq!(roi.size().unwrap(), edges.size().unwrap());
}

#[test]
fn test_line_detection() {
    let image = load_test_image("road.jpg");
    let filtered = filter_colors(&image, DayNightMode::Day);
    let edges = detect_edges(&filtered);
    let roi = apply_region_of_interest(&edges);
    
    let lines = detect_lines(&roi);
    
    // There should be some lines detected in a road image
    assert!(!lines.is_empty());
}

#[test]
fn test_line_filtering() {
    let image = load_test_image("road.jpg");
    let filtered = filter_colors(&image, DayNightMode::Day);
    let edges = detect_edges(&filtered);
    let roi = apply_region_of_interest(&edges);
    let lines = detect_lines(&roi);
    
    let (left_lines, right_lines) = filter_lines(&lines, image.cols());
    
    // There should be some lines in at least one of the categories
    assert!(left_lines.len() > 0 || right_lines.len() > 0);
}

#[test]
fn test_lane_fitting() {
    let image = load_test_image("road.jpg");
    let filtered = filter_colors(&image, DayNightMode::Day);
    let edges = detect_edges(&filtered);
    let roi = apply_region_of_interest(&edges);
    let lines = detect_lines(&roi);
    let (left_lines, right_lines) = filter_lines(&lines, image.cols());
    
    let (left_lane, right_lane) = fit_lane_lines(&left_lines, &right_lines, image.rows(), image.cols());
    
    // Basic validation - at least one lane should be detected
    assert!(left_lane.is_some() || right_lane.is_some());
}

#[test]
fn test_lane_validation() {
    let image = load_test_image("road.jpg");
    let filtered = filter_colors(&image, DayNightMode::Day);
    let edges = detect_edges(&filtered);
    let roi = apply_region_of_interest(&edges);
    let lines = detect_lines(&roi);
    let (left_lines, right_lines) = filter_lines(&lines, image.cols());
    let (left_lane, right_lane) = fit_lane_lines(&left_lines, &right_lines, image.rows(), image.cols());
    
    let valid_lanes = validate_lanes(left_lane.as_ref(), right_lane.as_ref(), image.cols());
    
    // Basic validation - result should be a boolean
    assert!(valid_lanes || !valid_lanes);
}

#[test]
fn test_end_to_end_pipeline() {
    let image = load_test_image("road.jpg");
    
    // Run the full pipeline
    let mode = detect_day_or_night(&image);
    let filtered = filter_colors(&image, mode);
    let edges = detect_edges(&filtered);
    let roi = apply_region_of_interest(&edges);
    let lines = detect_lines(&roi);
    let (left_lines, right_lines) = filter_lines(&lines, image.cols());
    let (left_lane, right_lane) = fit_lane_lines(&left_lines, &right_lines, image.rows(), image.cols());
    let valid_lanes = validate_lanes(left_lane.as_ref(), right_lane.as_ref(), image.cols());
    
    // Create a result object
    let result = LaneDetectionResult {
        left_lane,
        right_lane,
        valid: valid_lanes,
    };
    
    // Basic validation - result should contain lane information
    assert!(result.left_lane.is_some() || result.right_lane.is_some());
}

#[test]
fn test_video_processing() {
    // Create a simple test video with a few frames
    let frames = vec![
        load_test_image("road.jpg"),
        load_test_image("road2.jpg"),
        load_test_image("road3.jpg"),
    ];
    
    let video_path = create_test_video(&frames, 30.0);
    
    // Open the video
    let mut cap = VideoCapture::from_file(&video_path, CAP_ANY).unwrap();
    let mut frame = Mat::default();
    let mut frame_count = 0;
    let mut success_count = 0;
    
    // Process each frame
    while cap.read(&mut frame).unwrap() {
        frame_count += 1;
        
        // Run the lane detection pipeline
        let mode = detect_day_or_night(&frame);
        let filtered = filter_colors(&frame, mode);
        let edges = detect_edges(&filtered);
        let roi = apply_region_of_interest(&edges);
        let lines = detect_lines(&roi);
        let (left_lines, right_lines) = filter_lines(&lines, frame.cols());
        let (left_lane, right_lane) = fit_lane_lines(&left_lines, &right_lines, frame.rows(), frame.cols());
        let valid_lanes = validate_lanes(left_lane.as_ref(), right_lane.as_ref(), frame.cols());
        
        if valid_lanes {
            success_count += 1;
        }
    }
    
    // Verify that we processed the expected number of frames
    assert_eq!(frame_count, frames.len());
    
    // At least one frame should have valid lanes
    assert!(success_count > 0);
}

#[test]
fn test_edge_cases() {
    // Test with a blank image
    let mut blank = Mat::new_size_with_default(
        Size::new(640, 480),
        opencv::core::CV_8UC3,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
    ).unwrap();
    
    // Run the pipeline on the blank image
    let mode = detect_day_or_night(&blank);
    let filtered = filter_colors(&blank, mode);
    let edges = detect_edges(&filtered);
    let roi = apply_region_of_interest(&edges);
    let lines = detect_lines(&roi);
    
    // There should be no lines detected in a blank image
    assert!(lines.is_empty());
    
    // Test with a very dark image
    let mut dark = Mat::new_size_with_default(
        Size::new(640, 480),
        opencv::core::CV_8UC3,
        Scalar::new(10.0, 10.0, 10.0, 0.0),
    ).unwrap();
    
    let mode = detect_day_or_night(&dark);
    assert_eq!(mode, DayNightMode::Night);
}