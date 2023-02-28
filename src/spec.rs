use std::fmt::{Debug, Formatter};

pub type Spec = MatMul;

/// x > 0, y > 0
#[derive(Copy, Clone, PartialEq)]
pub struct MatMul {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Eq for MatMul {}

impl MatMul {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Debug for MatMul {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatMul({}x{})", self.x, self.y)
    }
}
