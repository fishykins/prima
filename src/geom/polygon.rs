use super::{Float, Line2, Triangle, Vec2, PI};

/// A complex polygon, defined by an array of vertices. It can be built from empty, as an ngon of n sides or with a set of verticies.
/// # Examples
///
/// ```
/// let poly = Polygon::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 8.0), Vec2::new(4.0, 8.0), Vec2::new(8.0, 8.0), Vec2::new(8.0, 0.0));
/// assert_eq!(poly.n(), 5);
/// 
/// let poly2 = Polygon::new_ngon(Vec2::new(32.0, 32.0), 16.0, 8);
/// assert_eq!(poly2.n(), 8);
/// ```
pub struct Polygon {
    verticies: Vec<Vec2>,
}

impl Polygon {
    /// Builds a new Polygon from the given Vec of points.
    pub fn new(verticies: Vec<Vec2>) -> Self {
        Self { verticies }
    }
    /// Creates a new Polygon with no points or edges assigned.
    pub fn empty() -> Self {
        Self {
            verticies: Vec::new(),
        }
    }

    /// Creates an empty Polygon with a set capacity for the number of points it may contain.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            verticies: Vec::with_capacity(capacity),
        }
    }

    /// Builds an ngon of equal length sides.
    pub fn new_ngon(pos: Vec2, circumradius: Float, n: usize) -> Self {
        if n < 3 {
            panic!("Polygon must have at least 3 sides");
        }

        let mut poly = Self {
            verticies: Vec::new(),
        };

        let angle = (2. * PI) / n as Float;

        for i in 0..n {
            // angle is ajusted by Pi/2 so triangulation starts from 12 O'clock
            let a = angle * i as Float + (PI / 2.);
            let x = a.cos() * circumradius;
            let y = a.sin() * circumradius;
            poly.verticies.push(Vec2::new(x, y) + pos);
        }
        poly
    }

    /// Adds a vertex to the polygon.
    pub fn add_vertex(&mut self, v: Vec2) {
        self.verticies.push(v);
    }

    /// The number of sides
    pub fn n(&self) -> usize {
        self.verticies.len()
    }

    /// Calculates the interior angle for a regular polygon of this size
    pub fn interior_angle(&self) -> Float {
        let n = self.verticies.len() as Float;
        ((n as Float - 2.) * PI) / n
    }

    /// Returns all vecticies in the polygon
    pub fn verticies(&self) -> Vec<Vec2> {
        self.verticies.clone()
    }

    /// Generates all edges
    pub fn edges(&self) -> Vec<Line2> {
        let mut lines = Vec::new();
        for (i, _) in self.verticies.iter().enumerate() {
            lines.push(self.edge(i, true).unwrap());
        }
        lines
    }

    /// Getter for vertex at given index
    pub fn vertex(&self, i: usize) -> Option<Vec2> {
        if i < self.verticies.len() {
            return Some(self.verticies[i]);
        }
        return None;
    }

    /// Returns true if polygon is convex
    pub fn is_convex(&self) -> bool {
        let n = self.verticies.len();
        if n < 3 {
            true
        } else {
            let mut i = 0;
            let l = n - 2;

            while i < l {
                let triangle = Triangle::new(
                    self.verticies[i],
                    self.verticies[i + 1],
                    self.verticies[i + 2],
                );
                if !triangle.is_convex() {
                    return false;
                } else {
                    i += 3;
                }
            }

            let triangle =
                Triangle::new(self.verticies[l], self.verticies[l + 1], self.verticies[0]);
            if !triangle.is_convex() {
                return false;
            }
            let triangle =
                Triangle::new(self.verticies[l + 1], self.verticies[0], self.verticies[1]);
            if !triangle.is_convex() {
                return false;
            }
            true
        }
    }

    /// Triangulates the polygon.
    /// impliments "Ear Clipping". See also: <https://gitlab.com/nathanfaucett/rs-polygon2/-/blob/master/src/triangulate.rs>
    pub fn triangulate(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let n = self.verticies.len();

        if n < 3 {
            //This is not going to triangulate- return nothing
            return triangles;
        }

        if n == 3 {
            //This IS a triangle, so simply return it as is
            triangles.push(Triangle::new(
                self.verticies[0],
                self.verticies[1],
                self.verticies[2],
            ));
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

        triangles.push(Triangle::new(
            self.verticies[avl[0]],
            self.verticies[avl[1]],
            self.verticies[avl[2]],
        ));
        triangles
    }

    /// Getter for edge, going from a given vertex (either clockwise or counter).
    pub fn edge(&self, i: usize, clockwise: bool) -> Option<Line2> {
        let vert_count = self.verticies.len();

        if clockwise {
            if i + 1 < vert_count {
                return Some(Line2 {
                    a: self.verticies[i],
                    b: self.verticies[i + 1],
                });
            } else if i < vert_count {
                return Some(Line2 {
                    a: self.verticies[i],
                    b: self.verticies[0],
                });
            }
        } else {
            if i < vert_count {
                if i > 0 {
                    return Some(Line2 {
                        a: self.verticies[i],
                        b: self.verticies[i - 1],
                    });
                } else {
                    return Some(Line2 {
                        a: self.verticies[i],
                        b: self.verticies[vert_count - 1],
                    });
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;
    use crate::geom::polygon::Polygon;

    const POLY_SIZE: usize = 12;

    #[test]
    fn polygon_test() {
        let poly = Polygon::new_ngon(Vec2::new(256., 256.), 200., POLY_SIZE);
        assert_eq!(poly.n(), POLY_SIZE)
    }
}
