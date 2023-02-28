use std::fmt::{Debug, Formatter};

pub(crate) type Spec = MatMul;

#[derive(Clone)]
pub(crate) struct MatMul {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Debug for MatMul {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatMul ({}x{})", self.x, self.y)
    }
}
