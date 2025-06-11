use std::path::Path;
use anyhow::{Context, Result};
use clap::{App, Arg};
use opencv::{
    core::Mat,
    highgui::{imshow, wait_key},
    imgcodecs::imread,
    imgcodecs::IMREAD_COLOR,
};

mod image_processing;
mod utils;
mod constants;

use image_processing::{
    is_day_time,
    region_of_interest,
    filter_colors,
    apply_grayscale,
    apply_gaussian_blur,
    apply_canny,
    hough_lines,
    draw_lanes,
};

fn main() -> Result<()> {
    let matches = App::new("Lane Detection")
        .version("1.0")
        .author("Lane Detection Team")
        .about("Detects lanes in road images")
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("FILE")
                .help("Sets the input image file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let image_path = matches.value_of("image").unwrap();
    
    if !Path::new(image_path).exists() {
        anyhow::bail!("Image file does not exist: {}", image_path);
    }

    let source_image = imread(image_path, IMREAD_COLOR)
        .context("Failed to load the image")?;

    if source_image.empty() {
        anyhow::bail!("Failed to load image or image is empty");
    }

    imshow("Original Image", &source_image)?;
    wait_key(1)?;

    let is_day = is_day_time(&source_image)?;
    println!("Detected lighting condition: {}", if is_day { "Day" } else { "Night" });

    let roi_image = region_of_interest(&source_image)?;
    imshow("Region of Interest", &roi_image)?;
    wait_key(1)?;

    let filtered_image = filter_colors(&roi_image, is_day)?;
    imshow("Color Filtered", &filtered_image)?;
    wait_key(1)?;

    let gray_image = apply_grayscale(&filtered_image)?;
    imshow("Grayscale", &gray_image)?;
    wait_key(1)?;

    let blurred_image = apply_gaussian_blur(&gray_image)?;
    imshow("Gaussian Blur", &blurred_image)?;
    wait_key(1)?;

    let edges = apply_canny(&blurred_image)?;
    imshow("Canny Edges", &edges)?;
    wait_key(1)?;

    let roi_edges = region_of_interest(&edges)?;
    imshow("ROI Edges", &roi_edges)?;
    wait_key(1)?;

    let lines = hough_lines(&roi_edges)?;
    
    let result_image = draw_lanes(&source_image, &lines)?;
    imshow("Lane Detection Result", &result_image)?;
    
    wait_key(0)?;
    
    Ok(())
}