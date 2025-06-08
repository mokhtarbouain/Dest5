

use opencv::{core, highgui, videoio};
use std::path::Path;

pub struct VideoCapture {
    capture: videoio::VideoCapture,
}

impl VideoCapture {
    pub fn new(file_path: &str) -> Result<VideoCapture, String> {
        if !Path::new(file_path).exists() {
            return Err(format!("File {} does not exist", file_path));
        }

        let capture = videoio::VideoCapture::from_file(file_path, videoio::CAP_ANY);
        match capture {
            Ok(capture) => {
                if !capture.is_opened() {
                    return Err(format!("Failed to open file {}", file_path));
                }
                Ok(VideoCapture { capture })
            }
            Err(err) => Err(format!("Failed to open file {}: {}", file_path, err)),
        }
    }

    pub fn is_opened(&self) -> bool {
        self.capture.is_opened()
    }

    pub fn read(&self) -> Result<Option<core::Mat>, String> {
        let mut frame = core::Mat::default();
        match self.capture.read(&mut frame) {
            true => {
                if frame.empty() {
                    Ok(None)
                } else {
                    Ok(Some(frame))
                }
            }
            false => Err("Failed to read frame".to_string()),
        }
    }
}