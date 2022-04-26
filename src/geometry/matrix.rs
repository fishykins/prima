/// A 2x2 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat2<N = f32> {
    // ===== TOP ===== //

    /// [0, 0]
    pub m00: N,
    /// [1, 0]
    pub m10: N,

    // === BOTTOM === //

    /// [0, 1]
    pub m01: N,
    /// [1, 1]
    pub m11: N,
}

impl<N> Mat2<N> {
    /// Creates a new matrix.
    pub fn new(m00: N, m10: N, m01: N, m11: N) -> Self {
        Self { m00, m01, m10, m11 }
    }
}
