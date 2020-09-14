use vek::{Vec2};
use std::f64::consts::PI;
use crate::core::OrdNum;
use super::line::*;
use super::Triangle;

pub struct Polygon<T> where T: OrdNum {
    verticies: Vec<Vec2<T>>,
}

impl<T> Polygon<T> where T: OrdNum {
    pub fn new_ngon(n: usize, circumradius: T, pos: Vec2<T>) -> Self {

        if n < 3 {
            panic!("Polygon must have at least 3 sides");
        }

        let mut poly = Self {
            verticies: Vec::new(),
        };

        let angle = (2. * PI) / n as f64; //regular_simple_angle(n);
        println!("angle = {}", angle);
        
        for i in 0..n {
            let a = angle * i as f64;
            let x = T::from_f64(a.cos()).unwrap() * circumradius;
            let y = T::from_f64(a.sin()).unwrap() * circumradius;
            poly.verticies.push(Vec2::new(x, y) + pos);
        }

        poly
    }

    pub fn n(&self) -> usize {
        self.verticies.len()
    }

    /// calculates the interior angle for a regular poly of our size
    pub fn interoir_angle(&self) -> f64 {
        let n = self.verticies.len() as f64;
        ((n as f64 - 2.) * PI) / n
    }

    pub fn verticies(&self) -> Vec<Vec2<T>> {
        self.verticies.clone()
    }

    pub fn edges(&self) -> Vec<Line<T>> {
        let mut lines = Vec::new();
        for (i, _) in self.verticies.iter().enumerate() {
            lines.push(self.edge(i, true).unwrap());
        }
        lines
    }

    pub fn vertex(&self, i: usize) -> Option<Vec2<T>> {
        if i < self.verticies.len() {
            return Some(self.verticies[i]);
        }
        return None;
    }

    /// impliments "Ear Clipping". See also: https://gitlab.com/nathanfaucett/rs-polygon2/-/blob/master/src/triangulate.rs
    pub fn triangulate(&self) -> Vec<Triangle<T>> {
        let mut triangles = Vec::new();
        let n = self.verticies.len();

        if n < 3 {
            //This is not going to triangulate- return nothing
            return triangles;
        }

        if n == 3 {
            //This IS a triangle, so simply return it as is
            triangles.push(Triangle::new(self.verticies[0], self.verticies[1], self.verticies[2]));
            return triangles;
        }

        //time to impliment "Ear Clipping". Wont work for complex polys, but meh. 
        let mut avl = Vec::with_capacity(n);

        for i in 0..n {
            avl.push(i);
        }

        let mut i = 0;
        let mut al = n;
        while al > 3 {
            let i0 = avl[i % al];
            let i1 = avl[(i + 1) % al];
            let i2 = avl[(i + 2) % al];

            let a = self.verticies[i0];
            let b = self.verticies[i1];
            let c = self.verticies[i2];

            let t = Triangle::new(a, b, c);

            let mut ear_found = false;
            if t.is_convex() {
                ear_found = true;

                for j in 0..al {
                    let vi = avl[j];

                    if vi != i0 && vi != i1 && vi != i2 {
                        if t.contains_point(self.verticies[vi]) {
                            ear_found = false;
                            break;
                        }
                    }
                }
            }

            if ear_found {
                triangles.push(t);
                avl.remove((i + 1) % al);
                al -= 1;
                i = 0;
            } else if i > 3 * al {
                break;
            } else {
                i += 1;
            }
        }

        triangles.push(Triangle::new(self.verticies[avl[0]], self.verticies[avl[1]], self.verticies[avl[2]]));
        triangles
    }

    pub fn edge(&self, i: usize, clockwise: bool) -> Option<Line<T>> {
        let vert_count = self.verticies.len();

        if clockwise {
            if i + 1 < vert_count {
                return Some(Line {
                    start: self.verticies[i],
                    end: self.verticies[i + 1],
                })
            }
            else if i < vert_count {
                return Some(Line {
                    start: self.verticies[i],
                    end: self.verticies[0],
                })
            }
        } else {
            if i < vert_count {
                if i > 0 {
                    return Some(Line {
                        start: self.verticies[i],
                        end: self.verticies[i - 1],
                    })
                } else {
                    return Some(Line {
                        start: self.verticies[i],
                        end: self.verticies[vert_count - 1],
                    })
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::polygon::Polygon;
    use crate::render::draw_line;
    use image::RgbImage;
    use vek::{Vec2,Rgb};

    const POLY_SIZE: usize = 7;

    #[test]
    fn polygon_test() {
        let mut img = RgbImage::new(512, 512);
        let poly = Polygon::new_ngon(POLY_SIZE, 128., Vec2::new(256., 256.));

        for (i, v) in poly.verticies().iter().enumerate() {
            println!("v{} => {}", i, v);
        }

        for v in poly.edges().iter() {
            let a = v.start.map(|x| x as u32);
            let b = v.end.map(|x| x as u32);
            let f1 = v.start.x as f64;
            let f2 = v.start.y as f64;
            let c1 = ((f2.sin() + 1.)  * 122.) as u8;
            let c2 = ((f1.cos() + 1.)* 122.) as u8;
            draw_line(&mut img, a, b, Rgb::new(c1,c2,0));
        }

        img.save("polygon_test.png").unwrap();
    }

    #[test]
    fn polygon_triangulation_test() {
        let mut img = RgbImage::new(512, 512);
        let poly = Polygon::new_ngon(POLY_SIZE, 128., Vec2::new(256., 256.));

        for triangle in poly.triangulate().iter() {
            let a = triangle.a.map(|x| x as u32);
            let b = triangle.b.map(|x| x as u32);
            let c = triangle.c.map(|x| x as u32);
            draw_line(&mut img, a, b, Rgb::new(0,255,0));
            draw_line(&mut img, b, c, Rgb::new(0,255,0));
            draw_line(&mut img, c, a, Rgb::new(0,255,0));
        }

        img.save("polygon_triangulation_test.png").unwrap();
    }
}