use crate::core::*;
use num::clamp as num_clamp;

pub fn clamp<T>(a: T, b: T, value: T) -> T where T: OrdNum {
    num_clamp(value, a, b)
}

pub fn clamp01<T>(value: T) -> T where T: OrdNum {
    clamp(T::zero(), T::one(), value)
}

/// Lerp between a and b by amount (0 - 1)
pub fn lerp<T>(a: T, b: T, amount: T) -> T where T: OrdNum {
    if a == b {
        return a;
    }
    if a > b {
        let range: T = a - b;
        return b + ( range * amount);
    } else {
        let range = b - a;
        return a + ( range * amount);
    }
}

/// Lerp & clamp between a and b by amount (0 - 1)
pub fn lerpc<T>(a: T, b: T, amount: T) -> T where T: OrdNum {
    lerp(a, b, clamp(T::zero(), T::one(), amount))
}

/// get the inverse lerp of a and b for value
pub fn inverse_lerp<T>(a: T, b: T, value: T) -> T where T: OrdNum {
    if a == b {
        return a;
    }
    if a > b {
        let range: T = a - b;
        return (value - b) / range;
    } else {
        let range = b - a;
        return (value - a) / range;
    }
}

#[test]
fn lerp_test() {
    assert_eq!(5., lerp(0., 10., 0.5));
    assert_eq!(0.5, inverse_lerp(0., 10., 5.0));
}