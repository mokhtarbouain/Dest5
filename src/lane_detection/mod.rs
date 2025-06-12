mod lane_detection {
    mod color_filtering;
    mod edge_detection;
    mod lane_fitting;
    mod line_detection;
    mod region_of_interest;
    mod utils;

    pub use color_filtering::{apply_day_color_filter, apply_night_color_filter};
    pub use edge_detection::detect_edges;
    pub use lane_fitting::fit_lane_lines;
    pub use line_detection::{detect_lines, filter_lines};
    pub use region_of_interest::apply_region_of_interest;
    pub use utils::{convert_to_grayscale, apply_gaussian_blur, is_night_condition, draw_lanes};

    use image::{DynamicImage, RgbImage};

    /// Processes a single frame for lane detection
    pub fn process_frame(frame: &DynamicImage) -> RgbImage {
        let mut img = frame.to_rgb8();
        
        // Determine day/night conditions
        let is_night = is_night_condition(&img);
        
        // Apply appropriate color filtering based on conditions
        if is_night {
            img = apply_night_color_filter(&img);
        } else {
            img = apply_day_color_filter(&img);
        }
        
        // Convert to grayscale
        let gray_img = convert_to_grayscale(&img);
        
        // Apply Gaussian blur
        let blurred_img = apply_gaussian_blur(&gray_img);
        
        // Detect edges
        let edges = detect_edges(&blurred_img);
        
        // Apply region of interest masking
        let masked_edges = apply_region_of_interest(&edges);
        
        // Detect and filter lines
        let lines = detect_lines(&masked_edges);
        let filtered_lines = filter_lines(&lines);
        
        // Fit lane lines
        let lane_lines = fit_lane_lines(&filtered_lines);
        
        // Draw lanes on the original frame
        let result = draw_lanes(&img, &lane_lines);
        
        result
    }
}