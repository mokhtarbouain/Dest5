```rust
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};

pub struct VideoCaptureManager {
    capture: VideoCapture,
}

impl VideoCaptureManager {
    pub fn new(video_path: &str) -> Result<Self, String> {
        let capture = VideoCapture::from_file(video_path, CAP_ANY)?;
        Ok(VideoCaptureManager { capture })
    }

    pub fn read_frame(&mut self) -> Result<Mat, String> {
        let mut frame = Mat::default()?;
        self.capture.read(&mut frame)?;
        Ok(frame)
    }

    pub fn release(&mut self) {
        self.capture.release();
    }
}
```