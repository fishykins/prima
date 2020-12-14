
use rusttype::{Font};
use std::fs::{metadata, File};
use std::io::Read;

/// A simple and rather fragile way of loading a font. 
pub fn load_font(filename: &str) -> Option<Font> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("font buffer overflow");
    Font::try_from_vec(buffer)
}

#[test]
pub fn font_test() {
    use imageproc::drawing::{draw_text_mut, draw_cross_mut};

    let mut image = image::RgbImage::new(200u32, 200u32);
    let red = super::RgbRaw([255u8, 0u8, 0u8]);

    //let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    //let font = Font::try_from_vec(font).unwrap();
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