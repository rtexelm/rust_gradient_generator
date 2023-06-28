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
        let pixels: Vec<Vec<Rgb<u8>>> = vec![vec![Rgb([0,0,0]); width as usize] height as usize];
        Image {
            width,
            height,
            pixels,
        }
    }
}
