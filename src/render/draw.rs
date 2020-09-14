use crate::geom::{Line, Rect, Triangle, Polygon, Disk};
use crate::core::OrdNum;
use super::{RgbImage, draw_line, draw_circle};
use vek::{Vec2, Rgb};
use num::ToPrimitive;

pub trait Draw<T> {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>);
}

impl<T> Draw<T> for Line<T> where T: ToPrimitive + Copy {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        draw_line(image, self.start.map(|x| x.to_u32().unwrap()), self.end.map(|x| x.to_u32().unwrap()), colour);
    }
}

impl<T, E> Draw<T> for Rect<T, E> where T: ToPrimitive + Copy, E: OrdNum {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let x = self.x.to_u32().unwrap();
        let y = self.y.to_u32().unwrap();
        let w = self.w.to_u32().unwrap();
        let h = self.h.to_u32().unwrap();
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

impl<T> Draw<T> for Triangle<T> where T: OrdNum {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let a = self.a.map(|x| x.to_u32().unwrap());
        let b = self.b.map(|x| x.to_u32().unwrap());
        let c = self.c.map(|x| x.to_u32().unwrap());
        draw_line(image, a, b, colour);
        draw_line(image, b, c, colour);
        draw_line(image, c, a, colour);
    }
}

impl<T> Draw<T> for Polygon<T> where T: OrdNum {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        for v in self.edges().iter() {
            let a = v.start.map(|x| x.to_u32().unwrap());
            let b = v.end.map(|x| x.to_u32().unwrap());
            draw_line(image, a, b, colour);
        }
    }
}

impl<T, E> Draw<T> for Disk<T, E> where T: OrdNum, E: OrdNum {
    fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        let r = self.radius.to_u32().unwrap();
        let p = self.center.map(|x| x.to_u32().unwrap());
        draw_circle(image, p, r, colour);
    }
}

// impl<T, E> Draw<T> for Ellipsis<T, E> {
//     fn draw(&self, image: &mut RgbImage, colour: Rgb<u8>) {
        
//     }
// }


#[test]
fn draw_test() {
    let mut img = RgbImage::new(512, 512);
    let poly = Polygon::new_ngon(Vec2::new(256., 256.), 200., 5);
    
    let disk = Disk::new(Vec2::new(256.,256.), 200.);
    let rect = disk.rect();

    poly.draw(&mut img, Rgb::new(255,255,0));
    rect.draw(&mut img, Rgb::new(0,255,255));
    disk.draw(&mut img, Rgb::new(255,0,255));

    img.save("draw_test.png").unwrap();
}