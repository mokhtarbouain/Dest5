use anyhow::Result;
use opencv::{
    core::{bitwise_and, bitwise_or, Mat, Scalar},
    imgproc::{cvt_color, in_range, COLOR_BGR2HSV},
    prelude::*,
};

use crate::constants::{WHITE_LOWER, WHITE_UPPER, YELLOW_LOWER, YELLOW_UPPER};

pub fn filter_colors(source: &Mat, is_day: bool) -> Result<Mat> {
    let mut hsv = Mat::default();
    cvt_color(&source, &mut hsv, COLOR_BGR2HSV, 0)?;

    let mut white_mask = Mat::default();
    let mut yellow_mask = Mat::default();
    
    in_range(
        &hsv,
        &Scalar::from(WHITE_LOWER),
        &Scalar::from(WHITE_UPPER),
        &mut white_mask,
    )?;
    
    in_range(
        &hsv,
        &Scalar::from(YELLOW_LOWER),
        &Scalar::from(YELLOW_UPPER),
        &mut yellow_mask,
    )?;

    let mut combined_mask = Mat::default();
    bitwise_or(&white_mask, &yellow_mask, &mut combined_mask, &Mat::default())?;

    let mut filtered_image = Mat::default();
    bitwise_and(&source, &source, &mut filtered_image, &combined_mask)?;

    Ok(filtered_image)
}