use std::fmt;
use std::hash::Hash;

/// The standard indexing type to use if none is specified.
pub type DefaultIx = usize;

/// Black magic that allows most index type ints to be converted into usize while acting as generics.
pub unsafe trait IndexType: Copy + Default + Hash + Ord + fmt::Debug + 'static {
    /// Builds the IndexType from usize x.
    fn new(x: usize) -> Self;
    /// Converts the IndexType back into a usize.
    fn index(&self) -> usize;
    /// Gets the maximum value of IndexType.
    fn max() -> Self;
}


unsafe impl IndexType for usize {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x
    }
    #[inline(always)]
    fn index(&self) -> Self {
        *self
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::usize::MAX
    }
}

unsafe impl IndexType for u32 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u32
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u32::MAX
    }
}

unsafe impl IndexType for u16 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u16
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u16::MAX
    }
}

unsafe impl IndexType for u8 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u8
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u8::MAX
    }
}