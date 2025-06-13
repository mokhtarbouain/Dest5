```rust
use clap::{App, Arg};
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};

fn main() {
    let matches = App::new("Video Processor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Processes a video file")
        .arg(Arg::with_name("video")
            .help("Path to the video file")
            .required(true)
            .index(1))
        .get_matches();

    let video_path = matches.value_of("video").unwrap();
    let mut cap = VideoCapture::from_file(video_path, CAP_ANY).unwrap();

    if !cap.is_opened().unwrap() {
        eprintln!("Error: Could not open video file.");
        std::process::exit(-1);
    }

    loop {
        let mut frame = Mat::default()?;
        cap.read(&mut frame)?;

        if frame.size()?.width == 0 {
            break;
        }

        // Process the frame here
        // Example: Display the frame
        opencv::highgui::imshow("Frame", &frame)?;
        if opencv::highgui::wait_key(1)? == 27 {
            break;
        }
    }

    cap.release()?;
}
```