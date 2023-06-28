#![allow(dead_code)]
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Rgb<u8>>>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels: Vec<Vec<Rgb<u8>>> = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];
        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn new_from_file(path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgb8();

        let width = img.width();
        let height = img.height();

        let mut pixels = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];

        for (x, y, pixel) in img.enumerate_pixels() {
            pixels[y as usize][x as usize] = *pixel;
        }

        Ok(Image {
            width,
            height,
            pixels,
        })
    }

    pub fn generate_gradient_image(&mut self, color1: [u8; 3], color2: [u8; 3]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let r = ((color2[0] as f32 - color1[0] as f32) * (x as f32 / self.width as f32)
                    + color1[0] as f32) as u8;
                let g = ((color2[1] as f32 - color1[1] as f32) * (y as f32 / self.height as f32)
                    + color1[1] as f32) as u8;
                let b = ((color2[2] as f32 - color1[2] as f32) * (x as f32 + y as f32)
                    / (self.height as f32 + self.height as f32)
                    + color1[2] as f32) as u8;
            }
        }
    }
}
