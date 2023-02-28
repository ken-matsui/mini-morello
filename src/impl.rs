use crate::spec::MatMul;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub(crate) enum Impl {
    // Multiplication
    Mult,
    Loop { child: MatMul },
}

impl Default for Impl {
    fn default() -> Self {
        Impl::Mult
    }
}

impl Debug for Impl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Impl::Mult => write!(f, "Mult"),
            Impl::Loop { child } => write!(f, "Loop ({:?})", child),
        }
    }
}
