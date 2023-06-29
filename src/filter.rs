#![allow(dead_code)]

use crate::image::Image;
use image::{imageops, RgbImage};

pub struct Filter;

impl Filter {
    pub fn blur(image: &mut Image, sigma: f32) -> RgbImage {
        imageops::blur(&image.to_rgb_image(), sigma)
    }

    pub fn emphasize_blue(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.width {
                let pixel = &mut image.pixels[y as usize][x as usize];
            }
        }
    }
}
