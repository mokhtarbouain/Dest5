```rust
use clap::{App, Arg};
use opencv::prelude::*;
use opencv::videoio;
use opencv::highgui;
use std::process::exit;

fn main() {
    let matches = App::new("Video Capture")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("Captures video from a webcam and displays it")
        .arg(Arg::with_name("camera")
            .short("c")
            .long("camera")
            .value_name("CAMERA_INDEX")
            .help("Sets the camera index")
            .takes_value(true))
        .get_matches();

    let camera_index = matches.value_of("camera").unwrap_or("0").parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Invalid camera index");
        exit(1);
    });

    let mut cap = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY);
    if cap.is_err() {
        eprintln!("Failed to initialize video capture");
        exit(1);
    }
    let mut cap = cap.unwrap();

    if !cap.is_opened().unwrap_or(false) {
        eprintln!("Unable to open video capture");
        exit(1);
    }

    let window_name = "Video Capture";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE).unwrap();

    loop {
        let mut frame = Mat::default().unwrap();
        if !cap.read(&mut frame).unwrap_or(false) {
            eprintln!("Failed to read frame");
            break;
        }

        if frame.size()?.width > 0 {
            highgui::imshow(window_name, &frame).unwrap();
        }

        if highgui::wait_key(10).unwrap() == 27 {
            break;
        }
    }

    cap.release().unwrap();
    highgui::destroy_all_windows().unwrap();
}
```