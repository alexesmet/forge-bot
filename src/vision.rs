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

use super::conf;


pub struct Visor {
    width: u32,
    height: u32,
    capturer: scrap::Capturer,
    config: conf::Config,
}

pub enum VisorError {
    IO (std::io::Error),
    CV (opencv::Error)
}

impl Visor {
    pub fn new(config: conf::Config) -> Result<Self, std::io::Error> {
        let screen = scrap::Display::primary()?;
        return Ok(Visor {
            width:      screen.width() as u32,
            height:     screen.height() as u32,
            capturer:   scrap::Capturer::new(screen)?,
            config:     config
        });
    }

    pub fn get_field_for_screen(&self, field: &core::Mat) -> Result<[[usize; conf::FIELD_SIZE.1]; conf::FIELD_SIZE.0], opencv::Error> {
        let mut result = [[0; conf::FIELD_SIZE.1]; conf::FIELD_SIZE.0];
        for i in 0..conf::FIELD_SIZE.0 {
            for j in 0..conf::FIELD_SIZE.1 {
                let roi_rect = Rect_::new(
                    self.config.cell_size as i32 * i as i32,
                    self.config.cell_size as i32 * j as i32,
                    self.config.cell_size as i32,
                    self.config.cell_size as i32,
                );
                let mask = core::Mat::new_rows_cols_with_default(
                    self.config.cell_size as i32,
                    self.config.cell_size as i32,
                    core::CV_8UC1, Scalar_::all(1.0)
                )?;
                let scalar = opencv::core::mean(&Mat::roi(&field, roi_rect)?, &mask)?;
                result[i][j] = if scalar.0[0] > 200.0 { 0 } else { 9 };
            }
        }
        return Ok(result);
    }

    /// Retruns B/W Mat, where white is clean board
    pub fn analyze_screen(&mut self) -> Result<opencv::core::Mat, opencv::Error> {
        // TODO: Take these values from config.toml
        let lower = Scalar_::new(106.0, 61.0, 145.0, 0.0);
        let upper = Scalar_::new(160.0, 206.0, 255.0, 0.0);
        loop {
            if let Ok(frm) = self.capturer.frame() {
                let image;
                // `unsafe` lets me use same memory where frm is, extremely efficient
                // Possible reason for memory leaks
                unsafe {
                    image = Mat::new_rows_cols_with_data(
                        self.height as i32, self.width as i32,
                        core::CV_8UC4,
                        frm.as_ptr() as *mut std::ffi::c_void,
                        0
                    )?;
                }
                let mut blurr = Mat::default()?;
                let mut hsv = Mat::default()?;
                let mut in_range = Mat::default()?;
                let mut dialated = Mat::default()?;
                let mut eroded = Mat::default()?;
                let roi_rect = Rect_::new(
                    self.config.zero_point[0] as i32,
                    self.config.zero_point[1] as i32,
                    self.config.cell_size as i32 * conf::FIELD_SIZE.0 as i32,
                    self.config.cell_size as i32 * conf::FIELD_SIZE.1 as i32
                );
                let image = Mat::roi(&image, roi_rect)?;
                imgproc::gaussian_blur(&image, &mut blurr, core::Size::new(11,11),0.0,0.0, 2)?;
                imgproc::cvt_color(
                    &blurr, &mut hsv,
                    imgproc::ColorConversionCodes::COLOR_BGR2HSV as i32, 0
                )?;
                core::in_range(&hsv, &lower, &upper, &mut in_range);

                imgproc::dilate(&in_range,
                    &mut dialated,
                    &Mat::default()?,
                    core::Point_::new(-1,-1),
                    6, 1,
                    Scalar_::all(0.0)
                );

                imgproc::erode(&dialated,
                    &mut eroded,
                    &Mat::default()?,
                    core::Point_::new(-1,-1),
                    10, 1,
                    Scalar_::all(0.0)
                );

                return Ok(eroded);
            } else {
                std::thread::yield_now();
            }
        }
    }
}
