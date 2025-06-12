use clap::{App, Arg};
use opencv::{
    core,
    highgui,
    prelude::*,
    videoio::{self, VideoCapture, CAP_ANY},
};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Video Processor")
        .version("1.0")
        .author("Senior Developer")
        .about("Processes video files")
        .arg(
            Arg::with_name("input")
                .help("Input video file path")
                .required(true)
                .index(1),
        )
        .get_matches();

    let video_path = matches.value_of("input").unwrap();

    println!("Opening video file: {}", video_path);

    let mut cap = VideoCapture::from_file(video_path, CAP_ANY)?;

    if !cap.is_opened()? {
        eprintln!("Error: Could not open video file: {}", video_path);
        process::exit(1);
    }

    let window_name = "Video Processor";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;

    let mut frame = Mat::default();

    while cap.read(&mut frame)? {
        if frame.empty() {
            break;
        }

        highgui::imshow(window_name, &frame)?;

        let key = highgui::wait_key(30)?;
        if key > 0 && (key as u8) as char == 'q' {
            println!("Exiting on user request");
            break;
        }
    }

    println!("Video processing complete");
    Ok(())
}