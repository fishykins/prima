mod draw_line;
mod draw_circle;
mod text;

pub mod draw;

pub use draw_line::draw_line;
pub use draw_line::draw_line_segment;
pub use draw_circle::draw_circle;
pub use draw::Draw;
pub use text::{load_font, draw_text};

pub use image::{RgbImage, ImageBuffer, Pixel, Rgb as RgbRaw};

/// A wrapper function for pixel manipulation on an image
/// reverses the y value! A better alternative to flipping the while image later on
pub fn paint_pixel(image: &mut RgbImage, x: u32, y: u32, rgb: RgbRaw<u8>) {
    //println!("Painting [{},{}]", x, y);
    if x < image.width() && y < image.height() && y > 0 && x > 0 {
        image.put_pixel(x, image.height() - y, rgb);
    }
}

use rand::Rng;
use vek::Rgb;

pub fn random_colour<R>(rng: &mut R) -> Rgb<u8> where R: Rng {
    Rgb::new(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255))
}