// use crate::r#impl::matmuls::MatmulHole;
// use crate::r#impl::Impl;
use crate::specs::Spec;

use anyhow::Result;

pub fn dp(spec: Spec, memory_limits: Vec<i32>) -> Result<Option<Box<dyn Impl>>> {
    if let Some(imp) = dp_table.get(&spec, memory_limits) {
        return Ok(Some(imp));
    };

    let mut best_impl: Option<Box<dyn Impl>> = None;
    let mut best_impl_cost = None;
    let hole = MatmulHole::try_from(spec)?;
    for imp in hole.actions() {
        // Check if `impl` would consume too much memory. If so, continue.

        // Fill in any nested Specs.
        let best_subimpl = dp(imp.child, new_mlims)?;
        impl_cost = cost_fn(imp);
        if best_impl_cost == None || best_impl_cost > impl_cost {
            best_impl = imp;
            best_impl_cost = impl_cost;
        }
    }
    dp_table[(spec, mlims)] = best_impl;
    Ok(best_impl)
}
