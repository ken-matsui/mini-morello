use crate::dtypes::DType;

pub struct TensorSpec {
    pub dim_sizes: Vec<u32>,
    dtype: DType,
    contiguous_abs: usize,
    aligned: bool,
    bank: Option<String>,
    // layout: Option<layouts.Layout>,
    vector_shape: Option<Vec<u32>>,
}

impl Default for TensorSpec {
    // let t = Tensor { dim_sizes: vec![1, 2], ..Default::default() };
    fn default() -> TensorSpec {
        TensorSpec {
            dim_sizes: vec![],
            dtype: DType::Uint8,
            contiguous_abs: 0,
            aligned: true,
            bank: None,
            // layout: None,
            vector_shape: None,
        }
    }
}
