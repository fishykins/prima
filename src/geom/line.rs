use vek::{Aabr};

// re-export LineSegment2 as Line, along with extended functionality. 
// ! There seems to be a pull request relating to this functionality, so might be added natively to vek at some point
pub use vek::LineSegment2 as Line;

pub trait LineExt<T> where T: PartialOrd + Copy {
    fn boundingbox(&self) -> Aabr<T>;
}

impl<T> LineExt<T> for Line<T>  where T: PartialOrd + Copy {
    fn boundingbox(&self) -> Aabr<T> {
        Aabr {
            min: self.start,
            max: self.end,
        }.made_valid()
    }
}