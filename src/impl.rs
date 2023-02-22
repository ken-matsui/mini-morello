use std::collections::HashSet;

use crate::specs::tensor_spec::TensorSpec;
use crate::tensor::Tile;

pub enum Impl {
    Loop {
        spec: Box<Impl>,
        subscripts: Vec<i32>,
        _operands_subscripts: Vec<Vec<i32>>,
        tiles: HashSet<Tile>,
        inner_args: HashSet<Tile>,
        inner: Box<Impl>,
        parallel: bool,
    },
    Move {
        src: TensorSpec,
        dst: TensorSpec,
    },
}
