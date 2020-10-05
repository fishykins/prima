use crate::geom::{Line, Rect, Triangle, Polygon, Disk, Ellipsis};
use crate::core::OrdNum;
use super::{RgbImage, draw_line, draw_circle, paint_pixel, RgbRaw};
use vek::{Vec2, Rgb};
use num::Signed;

pub trait Draw<T> {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>);
}

impl<T> Draw<T> for Line<T> where T: OrdNum + Signed {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        draw_line(image, self.start.map(|x| x.to_i32().unwrap()), self.end.map(|x| x.to_i32().unwrap()), colour);
    }
}

impl<T, E> Draw<T> for Rect<T, E> where T: OrdNum + Signed, E: OrdNum + Signed {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let x = self.x.to_i32().unwrap();
        let y = self.y.to_i32().unwrap();
        let w = self.w.to_i32().unwrap();
        let h = self.h.to_i32().unwrap();
        let a = Vec2::new(x, y);
        let b = Vec2::new(x + w, y);
        let c = Vec2::new(x, y + h);
        let d = Vec2::new(x + w, y + h);
        draw_line(image, a, b, colour);
        draw_line(image, a, c, colour);
        draw_line(image, b, d, colour);
        draw_line(image, c, d, colour);
    }
}

impl<T> Draw<T> for Triangle<T> where T: OrdNum + Signed {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let a = self.a.map(|x| x.to_i32().unwrap());
        let b = self.b.map(|x| x.to_i32().unwrap());
        let c = self.c.map(|x| x.to_i32().unwrap());
        draw_line(image, a, b, colour);
        draw_line(image, b, c, colour);
        draw_line(image, c, a, colour);
    }
}

impl<T> Draw<T> for Polygon<T> where T: OrdNum + Signed {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        for v in self.edges().iter() {
            let a = v.start.map(|x| x.to_i32().unwrap());
            let b = v.end.map(|x| x.to_i32().unwrap());
            draw_line(image, a, b, colour);
        }
    }
}

impl<T, E> Draw<T> for Disk<T, E> where T: OrdNum + Signed, E: OrdNum + Signed {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let r = self.radius.to_i32().unwrap();
        let p = self.center.map(|x| x.to_i32().unwrap());
        draw_circle(image, p, r, colour);
    }
}

impl<T, E> Draw<T> for Ellipsis<T, E> where T: OrdNum + Signed, E: OrdNum + Signed {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        // just use the formula for an elipse
        let center = self.center.map(|x| x.to_f64().unwrap());
        let major = self.radius.w.to_f64().unwrap();
        let minor = self.radius.h.to_f64().unwrap();
        
        for xi in (-major as i64)..(major as i64) {
            let x = xi as f64;
            let y = (minor.powi(2) - ((x.powi(2) * minor.powi(2)) / major.powi(2))).sqrt();
            paint_pixel(image, (center.x + x) as u32, (center.y + y) as u32, RgbRaw([colour.r, colour.g, colour.b]));
            paint_pixel(image, (center.x + x) as u32, (center.y - y) as u32, RgbRaw([colour.r, colour.g, colour.b]));
        }

        // iterate both axis to avoid deadzones. There is 100% a neater way of doing this but meh
        for yi in (-minor as i64)..(minor as i64) {
            let y = yi as f64;
            let x = (major.powi(2) - ((y.powi(2) * major.powi(2)) / minor.powi(2))).sqrt();
            paint_pixel(image, (center.x - x) as u32, (center.y + y) as u32, RgbRaw([colour.r, colour.g, colour.b]));
            paint_pixel(image, (center.x + x) as u32, (center.y + y) as u32, RgbRaw([colour.r, colour.g, colour.b]));
        }
    }
}


#[test]
fn draw_test() {
    let mut img = RgbImage::new(512, 512);

    let poly = Polygon::new_ngon(Vec2::new(256., 256.), 200., 7);
    let disk = Disk::new(Vec2::new(256.,256.), 200.);
    let rect = disk.rect();

    let elpipse = Ellipsis {
        center: Vec2::new(256.,256.),
        radius: vek::Extent2::new(128., 64.),
    };

    poly.draw(&mut img, Rgb::new(255,255,0));
    disk.draw(&mut img, Rgb::new(255,0,255));
    rect.draw(&mut img, Rgb::new(0,255,255));
    elpipse.draw(&mut img, Rgb::new(0,0,255));

    //image::imageops::blur(&img, 0.9).save("draw_test.png").unwrap();
    img.save("draw_test.png").unwrap();
}