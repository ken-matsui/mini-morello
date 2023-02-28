use crate::specs::tensor_spec::TensorSpec;

pub trait TensorLike {
    fn spec(&self) -> TensorSpec;

    fn dim_sizes(&self) -> Vec<u32> {
        self.spec().dim_sizes
    }
}

pub struct Tile {
    _spec: TensorSpec,
}

impl TensorLike for Tile {
    fn spec(&self) -> TensorSpec {
        self._spec.clone()
    }
}
