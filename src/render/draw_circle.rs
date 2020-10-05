use crate::core::OrdNum;
use super::{RgbImage, draw_line};
use vek::{Vec2, Rgb};
use num::{Signed};
use std::f64::consts::PI;

const NUM_STEPS: usize = 128;

/// fake circle, using a many sides polygon.
pub fn draw_circle<T>(image: &mut RgbImage, point: Vec2<T>, radius: T, colour: Rgb<u8>) 
where 
    T: OrdNum + Signed {

    let angle = (2. * PI) / NUM_STEPS as f64; 
    let r: f64 = radius.to_f64().unwrap();
    let px = point.x.to_f64().unwrap();
    let py = point.y.to_f64().unwrap();
    
    let mut a: f64 =  PI / 2.;
    let mut x = a.cos() * r + px;
    let mut y = a.sin() * r + py;
    let mut last_pos = Vec2::new(T::from_f64(x).unwrap(), T::from_f64(y).unwrap());

    for i in 0..=NUM_STEPS {
        a = angle * i as f64 + (PI / 2.);
        x = a.cos() * r + px;
        y = a.sin() * r + py;

        let pos = Vec2::new(T::from_f64(x).unwrap(), T::from_f64(y).unwrap());
        if last_pos != Vec2::zero() {
            draw_line(image, last_pos, pos, colour);
        }
        
        last_pos = pos;
    }
}