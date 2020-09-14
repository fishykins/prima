use super::{RgbImage, RgbRaw, paint_pixel};
use crate::geom::{Line, LineExt};
use vek::{Vec2, Aabr, Extent2, Rgb};
use num::{Num, Integer, Unsigned, ToPrimitive};


pub fn draw_line<T>(image: &mut RgbImage, a: Vec2<T>, b: Vec2<T>, colour: Rgb<u8>) where T: Num + Integer + Unsigned + ToPrimitive + Copy {
    let line = Line {
        start: a,
        end: b,
    };
    draw_line_segment(image, &line, colour);
}

/// Draws a pixel-level line onto the given image
pub fn draw_line_segment<T>(image: &mut RgbImage, line: &Line<T>, colour: Rgb<u8>) where T: Num + Integer + Unsigned + ToPrimitive + Copy {
    if line.start == line.end {
        return;
    }

    let line_box = line.boundingbox().map(|x| x.to_u32().unwrap());

    let image_box = Aabr {
        min: Vec2::zero(),
        max: Vec2::new(image.width(), image.height()),
    }.made_valid();

    if !image_box.contains_aabr(line_box) || line_box.size() == Extent2::zero() {
        return;
    }

    let rgb = RgbRaw([colour.r, colour.g, colour.b]);


    if line_box.size().w == 0 {
        // the width is 0 so its a vertical line.
        let x = line_box.min.x;
        for y in line_box.min.y..line_box.max.y {
            paint_pixel(image, x, y, rgb);
        }
    } else if line_box.size().h == 0 {
        // the height is 0 so its a horizontal line.
        let y = line_box.min.y;
        for x in line_box.min.x..line_box.max.x {
            paint_pixel(image, x, y, rgb);
        }
    } else {
        // the line has a gradient so we need to work that out
        let x_start = line.start.x.to_i64().unwrap();
        let y_start = line.start.y.to_i64().unwrap();
        let x_end = line.end.x.to_i64().unwrap();
        let y_end = line.end.y.to_i64().unwrap();

        let w = (x_end - x_start) as f64;
        let h = (y_end - y_start) as f64;
        let m: f64 = h / w;
        let c = y_start as f64 - (x_start as f64 * m);

        //println!("raw:  w = {}, h = {}, m = {}, c = {}", w, h, m, c);
        //println!("bbox: w = {}, h = {}", line_box.size().w, line_box.size().h);

        if line_box.size().w > line_box.size().h {
            //the line is more horizontal than vertical, so iterate the x axis
            let mut min = x_start as u32;
            let mut max = x_end as u32;

            // ensure we iterate from low to high
            if x_end < x_start {
                min = x_end as u32;
                max = x_start as u32;
            }

            //println!("x min/max = {}, {}", min, max);

            for x in min..max {
                let y = ((m * (x as f64)) + c) as u32;
                paint_pixel(image, x, y, rgb);
            }
        } else {
            //the line is more vertical than horizontal, so iterate the y axis
            let mut min = y_start as u32;
            let mut max = y_end as u32;

            // ensure we iterate from low to high
            if y_end < y_start {
                min = y_end as u32;
                max = y_start as u32;
            }

            //println!("y min/max = {}, {}", min, max);

            for y in min..max {
                let x = ((y as f64 - c) / m)  as u32;
                paint_pixel(image, x, y as u32, rgb);
            }
        }
    }
}

#[test]
fn draw_line_test() {
    let mut img = RgbImage::new(512, 512);

    let v1 = Vec2::new(500,290);
    let v2 = Vec2::new(234,63);
    let v3 = Vec2::new(512,0);

    //draw lines
    let l1: Line<u32> = Line {
        start: v1,
        end: v2,
    };
    let l2: Line<u32> = Line {
        start: v2,
        end: v3,
    };
    let l3: Line<u32> = Line {
        start: v3,
        end: v1,
    };

    draw_line_segment(&mut img, &l1, Rgb::new(255,0,0));
    draw_line_segment(&mut img, &l2, Rgb::new(0,255,0));
    draw_line_segment(&mut img, &l3, Rgb::new(0,0,255));

    img.save("line_test.png").unwrap();
}