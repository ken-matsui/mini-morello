use crate::dp::Elem;
use crate::dp_table::DpTablePtr;
use crate::r#impl::Impl;
use crate::spec::Spec;
use crate::util::dec;

pub(crate) type Cost = i32;

#[inline]
fn frac(a: usize, b: usize) -> f32 {
    a as f32 / b as f32
}
#[inline]
fn frac_ceil(a: usize, b: usize) -> i32 {
    frac(a, b).ceil() as i32
}

pub(crate) fn cost(imp: Impl, target_spec: Spec, dp: DpTablePtr<Elem>) -> Cost {
    match imp {
        Impl::Mult => 1, // Base Case
        Impl::Loop { child } => {
            let loop_cost = frac_ceil(target_spec.x, child.x) * frac_ceil(target_spec.y, child.y);
            let child_cost = unsafe { dp.get(dec(child.x), dec(child.y)).1 };
            // loop_cost * child_cost = this impl's cost
            (loop_cost * child_cost) as Cost
        }
    }
}
