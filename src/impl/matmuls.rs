use crate::r#impl::utils::gen_tile_sizes;
use crate::r#impl::Impl;
use crate::specs::Spec;

use anyhow::bail;

pub struct MatmulHole {
    spec: Spec,
}

impl MatmulHole {
    pub fn new(spec: Spec) -> Self {
        MatmulHole { spec }
    }

    pub fn actions(&self) -> Vec<Box<dyn Impl>> {
        let out = self.spec.out;

        // Search only over full line sizes
        for (h, w) in gen_tile_sizes(out.dim_sizes, Some(self._can_tile_out), true)? {}
    }
}

impl TryFrom<Spec> for MatmulHole {
    type Error = anyhow::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        if let Spec::Matmul { .. } = value {
            Ok(MatmulHole::new(value))
        }
        bail!("No hole type for: {value:?}");
    }
}
