

use opencv::{core, highgui, imgproc};
use opencv::prelude::*;
use std::path::Path;

struct LaneDetection;

impl LaneDetection {
    fn new() -> Self {
        LaneDetection
    }

    fn detect_lanes(&self, frame: &mut core::Mat) {
        let mut gray = core::Mat::default();
        imgproc::cvtColor(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0);

        let mut blurred = core::Mat::default();
        imgproc::GaussianBlur(gray, &mut blurred, core::Size::new(5, 5), 0, 0, imgproc::BORDER_DEFAULT);

        let mut edges = core::Mat::default();
        imgproc::Canny(blurred, &mut edges, 50, 150, 3, false);

        let mut lines = core::Mat::default();
        imgproc::HoughLinesP(edges, &mut lines, 1, core::PI / 180, 200, 0, 0);

        for line in lines.rows() {
            let line = line.at::<core::Vec4i>(0).unwrap();
            imgproc::line(frame, core::Point::new(line[0], line[1]), core::Point::new(line[2], line[3]), core::Scalar::new(0, 0, 255, 0), 2, imgproc::LINE_AA, 0);
        }
    }
}

fn main() {
    let lane_detection = LaneDetection::new();

    let video_path = "path_to_video_file";
    if !Path::new(video_path).exists() {
        eprintln!("Video file does not exist.");
        return;
    }

    let mut capture = highgui::VideoCapture::from_file(video_path, highgui::CAP_ANY).expect("Failed to open video file.");

    highgui::named_window("Detected Lanes", highgui::WINDOW_NORMAL).expect("Failed to create window.");
    highgui::resize_window("Detected Lanes", 800, 600).expect("Failed to resize window.");

    loop {
        let mut frame = core::Mat::default();
        if !capture.read(&mut frame).expect("Failed to read frame.") {
            break;
        }

        lane_detection.detect_lanes(&mut frame);

        highgui::imshow("Detected Lanes", &frame).expect("Failed to display frame.");

        if highgui::wait_key(1)? == 27 {
            break;
        }
    }
}