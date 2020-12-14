use crate::core::OrdNum;
use super::RgbRaw;
use image::{RgbImage};
use imageproc::drawing::{draw_text_mut};
use rusttype::{Font};
use std::fs::{metadata, File};
use std::io::Read;
use vek::Rgb;

/// A simple and rather fragile way of loading a font. 
pub fn load_font(filename: &str) -> Option<Font> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("font buffer overflow");
    Font::try_from_vec(buffer)
}

pub fn draw_text<T>(image: &mut RgbImage, x: T, y: T, text: &str, colour: Rgb<u8>, font: &Font) where T: OrdNum {
    let height = 12.4;
    let scale = rusttype::Scale {
        x: height * 2.0,
        y: height,
    };
    draw_text_mut(image, RgbRaw([colour.r, colour.g, colour.b]), x.to_u32().unwrap(), image.height() - y.to_u32().unwrap(), scale, &font, text);
}

#[test]
pub fn font_test() {
    use imageproc::drawing::{draw_text_mut, draw_cross_mut};

    let mut image = image::RgbImage::new(200u32, 200u32);
    let red = super::RgbRaw([255u8, 0u8, 0u8]);
    let font = load_font("assets/DejaVuSans.ttf").unwrap();
    let height = 12.4;
    let scale = rusttype::Scale {
        x: height * 2.0,
        y: height,
    };

    draw_cross_mut(&mut image, red, 5, 5);

    let text = "Hello, goon!";
    draw_text_mut(&mut image, red,0, 0, scale, &font, text);

    let _ = image.save("test.png").unwrap();
}