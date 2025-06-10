use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use approx::assert_relative_eq;
use lane_detection::{
    apply_canny, apply_gaussian_blur, apply_grayscale, average_slope_intercept, draw_lanes,
    filter_colors, hough_lines, is_day_time, process_image, region_of_interest, separate_lines,
};

fn create_test_image(width: u32, height: u32) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let color = if y > height / 2 {
                Rgb([100, 100, 100]) // Gray road
            } else {
                Rgb([135, 206, 235]) // Sky blue
            };
            img.put_pixel(x, y, color);
        }
    }
    
    // Add lane markings
    for y in height/2..height {
        // Left lane
        let left_x = width / 4;
        if (y as i32 - height as i32 / 2) % 20 < 10 {
            for x in left_x-2..left_x+2 {
                if x < width {
                    img.put_pixel(x, y, Rgb([255, 255, 255]));
                }
            }
        }
        
        // Right lane
        let right_x = 3 * width / 4;
        if (y as i32 - height as i32 / 2) % 20 < 10 {
            for x in right_x-2..right_x+2 {
                if x < width {
                    img.put_pixel(x, y, Rgb([255, 255, 255]));
                }
            }
        }
    }
    
    img
}

fn create_night_image(width: u32, height: u32) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let color = if y > height / 2 {
                Rgb([30, 30, 30]) // Dark road
            } else {
                Rgb([10, 10, 40]) // Dark sky
            };
            img.put_pixel(x, y, color);
        }
    }
    
    // Add lane markings with headlight effect
    for y in height/2..height {
        // Left lane
        let left_x = width / 4;
        if (y as i32 - height as i32 / 2) % 20 < 10 {
            for x in left_x-2..left_x+2 {
                if x < width {
                    img.put_pixel(x, y, Rgb([200, 200, 200]));
                }
            }
        }
        
        // Right lane
        let right_x = 3 * width / 4;
        if (y as i32 - height as i32 / 2) % 20 < 10 {
            for x in right_x-2..right_x+2 {
                if x < width {
                    img.put_pixel(x, y, Rgb([200, 200, 200]));
                }
            }
        }
    }
    
    img
}

fn compare_images(img1: &RgbImage, img2: &RgbImage) -> bool {
    if img1.dimensions() != img2.dimensions() {
        return false;
    }
    
    let (width, height) = img1.dimensions();
    for y in 0..height {
        for x in 0..width {
            if img1.get_pixel(x, y) != img2.get_pixel(x, y) {
                return false;
            }
        }
    }
    
    true
}

#[test]
fn test_is_day_time() {
    let day_image = create_test_image(640, 480);
    let night_image = create_night_image(640, 480);
    
    assert!(is_day_time(&DynamicImage::ImageRgb8(day_image)));
    assert!(!is_day_time(&DynamicImage::ImageRgb8(night_image)));
}

#[test]
fn test_region_of_interest() {
    let image = create_test_image(640, 480);
    let dynamic_image = DynamicImage::ImageRgb8(image);
    
    let roi = region_of_interest(&dynamic_image);
    
    // Check that ROI has same dimensions as original
    assert_eq!(roi.dimensions(), dynamic_image.dimensions());
    
    // Check that pixels outside ROI are black
    let (width, height) = roi.dimensions();
    for y in 0..height/3 {
        for x in 0..width {
            assert_eq!(roi.get_pixel(x, y), Rgb([0, 0, 0]));
        }
    }
}

#[test]
fn test_filter_colors() {
    let mut image = create_test_image(640, 480);
    
    // Add some yellow pixels
    for y in 300..320 {
        for x in 200..220 {
            image.put_pixel(x, y, Rgb([255, 255, 0]));
        }
    }
    
    let filtered = filter_colors(&DynamicImage::ImageRgb8(image));
    
    // Check that white lane markings are preserved
    let (width, height) = filtered.dimensions();
    let mut found_white = false;
    let mut found_yellow = false;
    
    for y in height/2..height {
        for x in 0..width {
            let pixel = filtered.get_pixel(x, y);
            if pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200 {
                found_white = true;
            }
            if pixel[0] > 200 && pixel[1] > 200 && pixel[2] < 50 {
                found_yellow = true;
            }
        }
    }
    
    assert!(found_white, "White lane markings should be preserved");
    assert!(found_yellow, "Yellow lane markings should be preserved");
}

#[test]
fn test_apply_grayscale() {
    let image = create_test_image(640, 480);
    let grayscale = apply_grayscale(&DynamicImage::ImageRgb8(image));
    
    // Check dimensions
    assert_eq!(grayscale.dimensions(), (640, 480));
    
    // Check that all pixels have R=G=B (grayscale property)
    let (width, height) = grayscale.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = grayscale.get_pixel(x, y);
            assert_eq!(pixel[0], pixel[1]);
            assert_eq!(pixel[1], pixel[2]);
        }
    }
}

#[test]
fn test_apply_gaussian_blur() {
    let image = create_test_image(640, 480);
    let grayscale = apply_grayscale(&DynamicImage::ImageRgb8(image));
    let blurred = apply_gaussian_blur(&grayscale);
    
    // Check dimensions
    assert_eq!(blurred.dimensions(), grayscale.dimensions());
    
    // Blurring should reduce the variance of neighboring pixels
    // This is a simple test to ensure the function is doing something
    let (width, height) = blurred.dimensions();
    let mut original_variance = 0.0;
    let mut blurred_variance = 0.0;
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let center_orig = grayscale.get_pixel(x, y)[0] as f32;
            let center_blur = blurred.get_pixel(x, y)[0] as f32;
            
            let neighbors_orig = [
                grayscale.get_pixel(x-1, y-1)[0] as f32,
                grayscale.get_pixel(x, y-1)[0] as f32,
                grayscale.get_pixel(x+1, y-1)[0] as f32,
                grayscale.get_pixel(x-1, y)[0] as f32,
                grayscale.get_pixel(x+1, y)[0] as f32,
                grayscale.get_pixel(x-1, y+1)[0] as f32,
                grayscale.get_pixel(x, y+1)[0] as f32,
                grayscale.get_pixel(x+1, y+1)[0] as f32,
            ];
            
            let neighbors_blur = [
                blurred.get_pixel(x-1, y-1)[0] as f32,
                blurred.get_pixel(x, y-1)[0] as f32,
                blurred.get_pixel(x+1, y-1)[0] as f32,
                blurred.get_pixel(x-1, y)[0] as f32,
                blurred.get_pixel(x+1, y)[0] as f32,
                blurred.get_pixel(x-1, y+1)[0] as f32,
                blurred.get_pixel(x, y+1)[0] as f32,
                blurred.get_pixel(x+1, y+1)[0] as f32,
            ];
            
            for &n in &neighbors_orig {
                original_variance += (center_orig - n).powi(2);
            }
            
            for &n in &neighbors_blur {
                blurred_variance += (center_blur - n).powi(2);
            }
        }
    }
    
    assert!(blurred_variance < original_variance, "Blurring should reduce variance between neighboring pixels");
}

#[test]
fn test_apply_canny() {
    let image = create_test_image(640, 480);
    let grayscale = apply_grayscale(&DynamicImage::ImageRgb8(image));
    let blurred = apply_gaussian_blur(&grayscale);
    let edges = apply_canny(&blurred);
    
    // Check dimensions
    assert_eq!(edges.dimensions(), blurred.dimensions());
    
    // Check that edges are detected (some pixels should be white)
    let (width, height) = edges.dimensions();
    let mut edge_pixels = 0;
    
    for y in 0..height {
        for x in 0..width {
            if edges.get_pixel(x, y)[0] > 0 {
                edge_pixels += 1;
            }
        }
    }
    
    assert!(edge_pixels > 0, "Canny edge detection should find some edges");
    assert!(edge_pixels < (width * height), "Not all pixels should be edges");
}

#[test]
fn test_hough_lines() {
    let image = create_test_image(640, 480);
    let grayscale = apply_grayscale(&DynamicImage::ImageRgb8(image));
    let blurred = apply_gaussian_blur(&grayscale);
    let edges = apply_canny(&blurred);
    let lines = hough_lines(&edges);
    
    // There should be some lines detected
    assert!(!lines.is_empty(), "Hough transform should detect some lines");
}

#[test]
fn test_separate_lines() {
    // Create some test lines
    let lines = vec![
        (100.0, 200.0, 150.0, 300.0),  // Positive slope (right)
        (400.0, 300.0, 350.0, 200.0),  // Negative slope (left)
        (200.0, 200.0, 250.0, 300.0),  // Positive slope (right)
        (500.0, 300.0, 450.0, 200.0),  // Negative slope (left)
    ];
    
    let (left_lines, right_lines) = separate_lines(&lines);
    
    assert_eq!(left_lines.len(), 2, "Should identify 2 left lines");
    assert_eq!(right_lines.len(), 2, "Should identify 2 right lines");
    
    // Check that lines are correctly categorized
    for &(x1, y1, x2, y2) in &left_lines {
        let slope = (y2 - y1) / (x2 - x1);
        assert!(slope < 0.0, "Left lines should have negative slope");
    }
    
    for &(x1, y1, x2, y2) in &right_lines {
        let slope = (y2 - y1) / (x2 - x1);
        assert!(slope > 0.0, "Right lines should have positive slope");
    }
}

#[test]
fn test_average_slope_intercept() {
    // Create some test lines
    let left_lines = vec![
        (400.0, 300.0, 350.0, 200.0),  // Slope = -2.0, intercept = 1100.0
        (500.0, 300.0, 450.0, 200.0),  // Slope = -2.0, intercept = 1300.0
    ];
    
    let right_lines = vec![
        (100.0, 200.0, 150.0, 300.0),  // Slope = 2.0, intercept = 0.0
        (200.0, 200.0, 250.0, 300.0),  // Slope = 2.0, intercept = -200.0
    ];
    
    let (width, height) = (640, 480);
    let result = average_slope_intercept(&left_lines, &right_lines, width, height);
    
    assert!(result.is_some(), "Should return Some result with valid lines");
    
    if let Some((left_line, right_line)) = result {
        // Check left line
        let (x1, y1, x2, y2) = left_line;
        assert!(x1 < x2, "Left line should go from right to left as y increases");
        assert!(y1 > y2, "Left line should go from bottom to top");
        
        // Check right line
        let (x1, y1, x2, y2) = right_line;
        assert!(x1 > x2, "Right line should go from left to right as y increases");
        assert!(y1 > y2, "Right line should go from bottom to top");
    }
}

#[test]
fn test_draw_lanes() {
    let image = create_test_image(640, 480);
    let left_line = (320.0, 480.0, 200.0, 300.0);
    let right_line = (320.0, 480.0, 440.0, 300.0);
    
    let result = draw_lanes(&DynamicImage::ImageRgb8(image), left_line, right_line);
    
    // Check that the result has the same dimensions
    assert_eq!(result.dimensions(), (640, 480));
    
    // Check that some red pixels exist (lane overlay)
    let (width, height) = result.dimensions();
    let mut found_red_pixels = false;
    
    for y in 0..height {
        for x in 0..width {
            let pixel = result.get_pixel(x, y);
            if pixel[0] > 200 && pixel[1] < 50 && pixel[2] < 50 {
                found_red_pixels = true;
                break;
            }
        }
        if found_red_pixels {
            break;
        }
    }
    
    assert!(found_red_pixels, "Lane overlay should contain red pixels");
}

#[test]
fn test_process_image_day() {
    let image = create_test_image(640, 480);
    let result = process_image(&DynamicImage::ImageRgb8(image));
    
    // Check that processing returns a result
    assert!(result.is_some(), "Processing day image should return Some result");
    
    if let Some(processed) = result {
        // Check dimensions
        assert_eq!(processed.dimensions(), (640, 480));
        
        // Check that some red pixels exist (lane overlay)
        let (width, height) = processed.dimensions();
        let mut found_red_pixels = false;
        
        for y in 0..height {
            for x in 0..width {
                let pixel = processed.get_pixel(x, y);
                if pixel[0] > 200 && pixel[1] < 50 && pixel[2] < 50 {
                    found_red_pixels = true;
                    break;
                }
            }
            if found_red_pixels {
                break;
            }
        }
        
        assert!(found_red_pixels, "Processed image should contain lane markings");
    }
}

#[test]
fn test_process_image_night() {
    let image = create_night_image(640, 480);
    let result = process_image(&DynamicImage::ImageRgb8(image));
    
    // Night images might not be processed successfully
    if let Some(processed) = result {
        // If processed, check dimensions
        assert_eq!(processed.dimensions(), (640, 480));
    }
    // It's okay if result is None for night images
}

#[test]
fn test_end_to_end_pipeline() {
    let test_image_path = Path::new("tests/test_data/sample_road.jpg");
    
    // Skip test if the test image doesn't exist
    if !test_image_path.exists() {
        println!("Test image not found, skipping end-to-end test");
        return;
    }
    
    let image = image::open(test_image_path).expect("Failed to open test image");
    let result = process_image(&image);
    
    assert!(result.is_some(), "End-to-end pipeline should process the test image");
    
    if let Some(processed) = result {
        // Check dimensions match original
        assert_eq!(processed.dimensions(), image.dimensions());
    }
}