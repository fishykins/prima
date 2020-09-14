use vek::{Aabr};
use super::Line;

// re-export LineSegment2 as Line, along with extended functionality. 
// ! There seems to be a pull request relating to this functionality, so might be added natively to vek at some point
pub trait LineExt<T> where T: PartialOrd + Copy {
    fn boundingbox(&self) -> Aabr<T>;
    fn reverse(&self) -> Self;
}

impl<T> LineExt<T> for Line<T>  where T: PartialOrd + Copy {
    fn boundingbox(&self) -> Aabr<T> {
        Aabr {
            min: self.start,
            max: self.end,
        }.made_valid()
    }

    fn reverse(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }
}