```rust
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};

pub struct VideoCapture {
    capture: VideoCapture,
}

impl VideoCapture {
    pub fn new() -> Result<Self, opencv::Error> {
        let capture = VideoCapture::new(0, CAP_ANY)?;
        Ok(VideoCapture { capture })
    }

    pub fn read_frame(&mut self) -> Result<Mat, opencv::Error> {
        let mut frame = Mat::default()?;
        self.capture.read(&mut frame)?;
        Ok(frame)
    }

    pub fn release(&mut self) {
        self.capture.release();
    }
}
```