use crate::cost::{cost, Cost};
use crate::dp_table::{DpTable, DpTablePtr};
use crate::r#impl::Impl;
use crate::spec::{MatMul, Spec};
use crate::util::{dec, inc};

use debug_print::debug_println as dprintln;
use threadpool::ThreadPool;

pub(crate) type Elem = (Impl, Cost);

pub(crate) fn compute_block(
    dp: DpTablePtr<Elem>,
    from_x: usize,
    to_x: usize,
    from_y: usize,
    to_y: usize,
) {
    for x in from_x..to_x {
        for y in from_y..to_y {
            if x == 0 && y == 0 {
                // Assume [0][0] is already calculated.
                continue;
            }
            let base_spec = Spec::new(inc(x), inc(y));

            let mut best_impl = Impl::default();
            let mut min_cost = Cost::MAX;

            for dep_x in 0..=x {
                for dep_y in 0..=y {
                    if dep_x == x && dep_y == y {
                        // skip itself
                        continue;
                    }

                    let dep_impl = Impl::Loop {
                        child: MatMul::new(inc(dep_x), inc(dep_y)),
                    };
                    let dep_cost = cost(base_spec, dep_impl.clone(), dp.clone());
                    dprintln!(
                        "base_spec: {base_spec:?}, dep_impl: {dep_impl:?}, dep_cost: {dep_cost}"
                    );

                    if min_cost >= dep_cost {
                        // >=: Take latter one if it has the same cost
                        // >: Use the first impl even if there is the same cost
                        min_cost = dep_cost;
                        best_impl = dep_impl;
                    }
                }
            }

            unsafe {
                dp.insert(x, y, (best_impl.clone(), min_cost));
            }
            dprintln!();
        }
    }
}

pub fn dp(spec: Spec, bsize: usize) -> Elem {
    #[allow(non_snake_case)]
    let X = spec.x;
    #[allow(non_snake_case)]
    let Y = spec.y;

    let mut dp = DpTable::<Elem>::new(X, Y);
    dp.insert(0, 0, (Impl::Mult, 1));
    let dp_p = dp.as_mut_ptr();

    let n_workers = if bsize > X || bsize > Y {
        1
    } else if X / bsize >= Y / bsize {
        X / bsize // 1000/100 = max 10 diagonal blocks in the middle
    } else {
        Y / bsize
    };
    let pool = ThreadPool::new(n_workers);

    let mut offset = 0;
    while offset <= (X + Y - 2) {
        let mut y = 0;
        while y <= offset {
            let dp_p = dp_p.clone();

            pool.execute(move || {
                let x = offset - y;
                if x <= X && y <= Y {
                    let to_x = if x + bsize < X { x + bsize } else { X };
                    let to_y = if y + bsize < Y { y + bsize } else { Y };
                    dprintln!("({x}, {y})..({to_x}, {to_y})");
                    compute_block(dp_p, x, to_x, y, to_y);
                }
            });

            y += bsize; // step_by
        }
        dprintln!();
        pool.join();

        offset += bsize; // step_by
    }
    dprintln!("\n{:?}", dp);

    dp.get(dec(X), dec(Y)).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        assert_eq!(dp(Spec::new(1, 1), 1), (Impl::Mult, 1));
        assert_eq!(
            dp(Spec::new(2, 2), 1),
            (
                Impl::Loop {
                    child: MatMul::new(2, 1)
                },
                4
            )
        );
        assert_eq!(
            dp(Spec::new(4, 4), 2),
            (
                Impl::Loop {
                    child: MatMul::new(4, 2)
                },
                16
            )
        );
    }
}
