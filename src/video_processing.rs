```rust
use opencv::{
    core,
    imgproc,
    prelude::*,
    videoio,
};

pub fn process_video(input_path: &str, output_path: &str) {
    let mut cap = videoio::VideoCapture::from_file(input_path, videoio::CAP_ANY).unwrap();
    let mut writer = videoio::VideoWriter::new(
        output_path,
        videoio::VideoWriter::fourcc('M', 'J', 'P', 'G'),
        cap.get(videoio::CAP_PROP_FPS).unwrap(),
        core::Size::new(
            cap.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap() as i32,
            cap.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap() as i32,
        ),
        true,
    ).unwrap();

    let mut frame = Mat::default();
    while cap.read(&mut frame).unwrap() {
        let processed_frame = process_frame(&frame);
        writer.write(&processed_frame).unwrap();
    }
}

fn process_frame(frame: &Mat) -> Mat {
    let mut gray_frame = Mat::default();
    imgproc::cvt_color(frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    let mut blurred_frame = Mat::default();
    imgproc::gaussian_blur(&gray_frame, &mut blurred_frame, core::Size::new(5, 5), 0.0, 0.0, core::BORDER_DEFAULT).unwrap();

    let mut edges = Mat::default();
    imgproc::canny(&blurred_frame, &mut edges, 50.0, 150.0, 3, false).unwrap();

    let mut masked_edges = Mat::default();
    let mut mask = Mat::default();
    core::in_range(&edges, &core::Scalar::new(255.0, 255.0, 255.0, 255.0), &core::Scalar::new(255.0, 255.0, 255.0, 255.0), &mut mask).unwrap();
    core::bitwise_and(&edges, &mask, &mut masked_edges).unwrap();

    let mut lines = Vec::new();
    imgproc::hough_lines_p(&masked_edges, &mut lines, 1.0, std::f64::consts::PI / 180.0, 50, 50.0, 10.0).unwrap();

    let mut lane_frame = frame.clone();
    for line in lines {
        imgproc::line(&mut lane_frame, core::Point::new(line[0], line[1]), core::Point::new(line[2], line[3]), core::Scalar::new(0.0, 255.0, 0.0, 0.0), 5, imgproc::LINE_AA, 0).unwrap();
    }

    lane_frame
}
```