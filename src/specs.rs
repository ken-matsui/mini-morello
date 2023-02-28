pub mod tensor_spec;

use tensor_spec::TensorSpec;

// pub enum Spec {
//     Matmul {
//         lhs: TensorSpec,
//         rhs: TensorSpec,
//         out: TensorSpec,
//     },
// }

pub trait Spec {
    fn output(&self) -> TensorSpec;

    fn inputs(&self) -> Vec<TensorSpec>;
}
