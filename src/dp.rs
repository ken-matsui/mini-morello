use crate::cost::{cost, Cost};
use crate::dp_table::{DpTable, DpTablePtr};
use crate::r#impl::Impl;
use crate::spec::{MatMul, Spec};
use crate::util::{dec, inc};

use std::cmp::min;

use debug_print::debug_println as dprintln;
use threadpool::ThreadPool;

pub(crate) type Elem = (Impl, Cost);

// serial
fn find_best_impl(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_x in 0..=x {
        for dep_y in 0..=y {
            for dep_z in 0..=z {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}

type ComputeBlockFn = fn(DpTablePtr<Elem>, usize, usize, usize, usize, usize, usize);
// serial
fn compute_block(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for x in from_x..to_x {
        for y in from_y..to_y {
            for z in from_z..to_z {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}

#[allow(non_snake_case)]
fn calc_workers(bsize: usize, X: usize, Y: usize, Z: usize) -> usize {
    if bsize > X || bsize > Y || bsize > Z {
        1
    } else {
        min(min(X, Y), Z) / bsize
    }
}

/// bsize: block size such that bsize^3
fn dp_impl(spec: Spec, bsize: usize, compute_block_fn: ComputeBlockFn) -> Elem {
    #[allow(non_snake_case)]
    let X = spec.x;
    #[allow(non_snake_case)]
    let Y = spec.y;
    #[allow(non_snake_case)]
    let Z = spec.z;

    let mut dp = DpTable::<Elem>::new(X, Y, Z);
    dp.insert(0, 0, 0, (Impl::Mult, 1));
    let dp_p = dp.as_mut_ptr();

    let pool = ThreadPool::new(calc_workers(bsize, X, Y, Z));
    for offset in (0..=(X + Y + Z - 3)).step_by(bsize) {
        for z in (0..=offset).step_by(bsize) {
            for y in (0..=(offset - z)).step_by(bsize) {
                let dp_p = dp_p.clone();

                pool.execute(move || {
                    let x = offset - y;
                    if x < X && y < Y && z < Z {
                        let to_x = if x + bsize < X { x + bsize } else { X };
                        let to_y = if y + bsize < Y { y + bsize } else { Y };
                        let to_z = if z + bsize < Z { z + bsize } else { Z };
                        dprintln!("({x}, {y}, {z})..({to_x}, {to_y}, {to_z})");
                        compute_block_fn(dp_p, x, to_x, y, to_y, z, to_z);
                    }
                });
            }
        }
        dprintln!();
        pool.join();
    }
    dprintln!("\n{:?}", dp);

    dp.get(dec(X), dec(Y), dec(Z)).clone()
}

pub fn dp(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block)
}

fn find_best_impl_xyz(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_x in 0..=x {
        for dep_y in 0..=y {
            for dep_z in 0..=z {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}
fn compute_block_xyz(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for x in from_x..to_x {
        for y in from_y..to_y {
            for z in from_z..to_z {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl_xyz(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}

fn find_best_impl_xzy(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_x in 0..=x {
        for dep_z in 0..=z {
            for dep_y in 0..=y {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}
fn compute_block_xzy(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for x in from_x..to_x {
        for z in from_z..to_z {
            for y in from_y..to_y {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl_xzy(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}

fn find_best_impl_yxz(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_y in 0..=y {
        for dep_x in 0..=x {
            for dep_z in 0..=z {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}
fn compute_block_yxz(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for y in from_y..to_y {
        for x in from_x..to_x {
            for z in from_z..to_z {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl_yxz(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}

fn find_best_impl_yzx(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_y in 0..=y {
        for dep_z in 0..=z {
            for dep_x in 0..=x {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}
fn compute_block_yzx(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for y in from_y..to_y {
        for z in from_z..to_z {
            for x in from_x..to_x {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl_yzx(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}

fn find_best_impl_zxy(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_z in 0..=z {
        for dep_x in 0..=x {
            for dep_y in 0..=y {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}
fn compute_block_zxy(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for z in from_z..to_z {
        for x in from_x..to_x {
            for y in from_y..to_y {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl_zxy(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}

fn find_best_impl_zyx(dp: DpTablePtr<Elem>, x: usize, y: usize, z: usize) -> (Impl, Cost) {
    let base_spec = Spec::new(inc(x), inc(y), inc(z));

    let mut best_impl = Impl::default();
    let mut min_cost = Cost::MAX;

    for dep_z in 0..=z {
        for dep_y in 0..=y {
            for dep_x in 0..=x {
                if dep_x == x && dep_y == y && dep_z == z {
                    // skip itself
                    continue;
                }

                let dep_impl = Impl::Loop {
                    child: MatMul::new(inc(dep_x), inc(dep_y), inc(dep_z)),
                };
                let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                dprintln!("base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}");

                if min_cost >= dep_cost {
                    // >=: Take latter one if it has the same cost
                    // >: Use the first impl even if there is the same cost
                    min_cost = dep_cost;
                    best_impl = dep_impl;
                }
            }
        }
    }

    (best_impl.clone(), min_cost)
}
fn compute_block_zyx(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
    from_z: usize,
    to_z: usize,
) {
    for z in from_z..to_z {
        for y in from_y..to_y {
            for x in from_x..to_x {
                if x == 0 && y == 0 && z == 0 {
                    // Assume that [0][0][0] is already calculated.
                    continue;
                }

                let (best_impl, min_cost) = find_best_impl_zyx(dp.clone(), x, y, z);
                unsafe {
                    dp.insert(x, y, z, (best_impl, min_cost));
                }
                dprintln!();
            }
        }
    }
}
pub fn dp_xyz(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block_xyz)
}
pub fn dp_xzy(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block_xzy)
}
pub fn dp_yxz(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block_yxz)
}
pub fn dp_yzx(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block_yzx)
}
pub fn dp_zxy(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block_zxy)
}
pub fn dp_zyx(spec: Spec, bsize: usize) -> Elem {
    dp_impl(spec, bsize, compute_block_zyx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        assert_eq!(dp(Spec::new(1, 1, 1), 1), (Impl::Mult, 1));
        assert_eq!(
            dp(Spec::new(2, 2, 1), 1),
            (
                Impl::Loop {
                    child: MatMul::new(2, 1, 1)
                },
                4
            )
        );
        assert_eq!(
            dp(Spec::new(4, 4, 1), 2),
            (
                Impl::Loop {
                    child: MatMul::new(4, 2, 1)
                },
                16
            )
        );
    }
}
