pub mod image_processing;
pub mod lane_analysis;
pub mod utils;

// Re-export public items from submodules
pub use image_processing::{
    preprocess_image,
    apply_threshold,
    detect_edges,
    apply_region_of_interest,
    perspective_transform,
};

pub use lane_analysis::{
    detect_lane_lines,
    calculate_lane_curvature,
    determine_vehicle_position,
    LaneDetectionResult,
    LaneParameters,
};

pub use utils::{
    draw_lane_overlay,
    convert_to_grayscale,
    resize_image,
    ImageSize,
};

// Module-level constants used across submodules
pub const DEFAULT_ROI_VERTICES_RATIO: [(f32, f32); 4] = [
    (0.0, 1.0),    // Bottom left
    (0.45, 0.6),   // Top left
    (0.55, 0.6),   // Top right
    (1.0, 1.0),    // Bottom right
];

pub const DEFAULT_PERSPECTIVE_SRC_POINTS_RATIO: [(f32, f32); 4] = [
    (0.43, 0.65),  // Top left
    (0.57, 0.65),  // Top right
    (0.85, 1.0),   // Bottom right
    (0.15, 1.0),   // Bottom left
];

pub const DEFAULT_PERSPECTIVE_DST_POINTS_RATIO: [(f32, f32); 4] = [
    (0.25, 0.0),   // Top left
    (0.75, 0.0),   // Top right
    (0.75, 1.0),   // Bottom right
    (0.25, 1.0),   // Bottom left
];

// Common types used across the module
pub enum LaneDetectionMethod {
    Hough,
    SlidingWindow,
    DeepLearning,
}

pub struct LaneDetectionConfig {
    pub detection_method: LaneDetectionMethod,
    pub roi_vertices_ratio: [(f32, f32); 4],
    pub perspective_src_points_ratio: [(f32, f32); 4],
    pub perspective_dst_points_ratio: [(f32, f32); 4],
    pub edge_detection_low_threshold: u8,
    pub edge_detection_high_threshold: u8,
}

impl Default for LaneDetectionConfig {
    fn default() -> Self {
        Self {
            detection_method: LaneDetectionMethod::SlidingWindow,
            roi_vertices_ratio: DEFAULT_ROI_VERTICES_RATIO,
            perspective_src_points_ratio: DEFAULT_PERSPECTIVE_SRC_POINTS_RATIO,
            perspective_dst_points_ratio: DEFAULT_PERSPECTIVE_DST_POINTS_RATIO,
            edge_detection_low_threshold: 50,
            edge_detection_high_threshold: 150,
        }
    }
}