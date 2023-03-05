use std::fmt::{Debug, Formatter};

pub type Spec = MatMul;

#[derive(Copy, Clone, PartialEq)]
pub struct MatMul {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) z: usize,
}

impl Eq for MatMul {}

impl MatMul {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        assert!(x > 0 && y > 0 && z > 0, "x,y,z must be >0");
        Self { x, y, z }
    }
}

impl Debug for MatMul {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatMul({}x{}x{})", self.x, self.y, self.z)
    }
}
