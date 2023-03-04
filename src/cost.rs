use crate::dp::Elem;
use crate::dp_table::DpTablePtr;
use crate::r#impl::Impl;
use crate::spec::Spec;
use crate::util::dec;

pub(crate) type Cost = i32;

#[inline]
fn frac(numerator: usize, denominator: usize) -> f32 {
    numerator as f32 / denominator as f32
}
#[inline]
fn frac_ceil(numerator: usize, denominator: usize) -> i32 {
    frac(numerator, denominator).ceil() as i32
}

pub(crate) fn cost(base_spec: Spec, imp: Impl, dp: DpTablePtr<Elem>) -> Cost {
    match imp {
        Impl::Mult => 1, // Base Case
        Impl::Loop { child } => {
            let loop_cost = frac_ceil(base_spec.x, child.x) * frac_ceil(base_spec.y, child.y);
            let child_cost = unsafe { dp.get(dec(child.x), dec(child.y), dec(child.z)).1 };
            // loop_cost * child_cost = this impl's cost
            (loop_cost * child_cost) as Cost
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frac() {
        assert!(f32::is_nan(frac(usize::MIN, usize::MIN)));
        assert_eq!(frac(usize::MAX, usize::MAX), 1_f32);
        assert_eq!(frac(usize::MIN, usize::MAX), 0_f32);
        assert!(f32::is_infinite(frac(usize::MAX, usize::MIN)));
        assert_eq!(frac(4, 1), 4_f32);
        assert_eq!(frac(4, 2), 2_f32);
        assert_eq!(frac(4, 3), 1.3333334_f32);
    }

    #[test]
    fn test_frac_ceil() {
        assert_eq!(frac_ceil(4, 1), 4_i32);
        assert_eq!(frac_ceil(4, 2), 2_i32);
        assert_eq!(frac_ceil(4, 3), 2_i32);
    }
}
