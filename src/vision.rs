#![allow(unused_imports)]
#![allow(dead_code)]

use scrap;

use image;

use opencv;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::core;
use opencv::core::Mat;
use opencv::core::Rect_;
use opencv::core::Scalar_;
use opencv::highgui;


use std::convert::TryInto;

use std::time::Instant;

use std::io::Cursor;
use std::io::prelude::*;
use std::fs::File;

use std::boxed::Box;

pub fn analyse_demo() {

    let screen = scrap::Display::primary().unwrap();
    let width: u32 = screen.width().try_into().unwrap();
    let height: u32 = screen.height().try_into().unwrap();
    let mut capturer = scrap::Capturer::new(screen).unwrap();


    let mut blurr = Mat::default().unwrap();
    let mut hsv = Mat::default().unwrap();
    let mut in_range = Mat::default().unwrap();
    let mut dialated = Mat::default().unwrap();
    let mut eroded = Mat::default().unwrap();

    let lower = Scalar_::new(106.0, 61.0, 145.0, 0.0);
    let upper = Scalar_::new(160.0, 206.0, 255.0, 0.0);

    loop {
        if let Ok(mut frm) = capturer.frame() { unsafe {
            let mut zeroes = [155 as u8; 40000];
            zeroes[0] = 255;
            zeroes[5] = 255;

            let image = Mat::new_rows_cols_with_data(
                height as i32, width as i32,
                core::CV_8UC4,
                // &mut zeroes as *mut _ as *mut std::ffi::c_void,
                frm.as_ptr() as *mut std::ffi::c_void,
                0
            ).unwrap();
            let image = Mat::roi(&image, Rect_::new(2, 413, 453 - 2, 795 - 413)).unwrap();
            imgproc::gaussian_blur(&image, &mut blurr, core::Size::new(11,11),0.0,0.0, 2).unwrap();
            imgproc::cvt_color(
                &blurr, &mut hsv, 
                imgproc::ColorConversionCodes::COLOR_BGR2HSV as i32, 0
            ).unwrap();
            core::in_range(&hsv, &lower, &upper, &mut in_range);

            imgproc::dilate(&in_range,
                &mut dialated,
                &Mat::default().unwrap(),
                core::Point_::new(-1,-1),
                5, 1,
                Scalar_::all(0.0)
            );

            imgproc::erode(&dialated,
                &mut eroded,
                &Mat::default().unwrap(),
                core::Point_::new(-1,-1),
                10, 1,
                Scalar_::all(0.0)
            );

            highgui::imshow("This", &eroded);
            highgui::wait_key(40);
        } }
    }
}
