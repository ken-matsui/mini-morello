use anyhow::{ensure, Result};

/// Returns possible dimension sizes up to `dim`.
///
/// If `tile_size_mode` is set to `CACHE_LINE_MULTIPLES`, returned sizes
/// will be evenly divisible by the system's cache line size.
///
/// If `tile_size_mode` is set to `POWERS_OF_TWO`, returned sizes
/// will be powers of two.
///
/// Arguments:
/// * `include_end` - If False, results will exclude `dim` itself.
pub fn dim_range(dim: i32, include_end: bool) -> Result<Vec<i32>> {
    ensure!(dim >= 0, "dim should be Z+");
    if dim == 0 {
        return Ok(vec![]);
    }

    // TODO: Support other TileSizeMode. Currently use POWERS_OF_TWO by default
    // https://github.com/samkaufman/morello/blob/302efb87037ef097a5dca55ffcb0da16b585b976/scripts/main.py#L43
    let mut result = vec![];
    let mut power = 0;
    loop {
        let pow = 2_i32.pow(power);
        if pow >= dim {
            break;
        }
        result.push(pow);
        power += 1;
    }
    if include_end {
        result.push(dim);
    }
    Ok(result)
}

/// Returns tile shapes to explore for a given tensor shape.
///
/// Doesn't return tensor_shape itself.
pub fn gen_tile_sizes(
    tensor_shape: Vec<i32>,
    filter: Option<fn(Vec<i32>) -> bool>,
    drop_given: bool,
) -> Result<Vec<Vec<i32>>> {
    let mut result = vec![];
    if tensor_shape.is_empty() {
        return Ok(vec![]);
    } else if tensor_shape.len() == 1 {
        for d in dim_range(tensor_shape[0], true)? {
            let new_shape = vec![d];
            if drop_given && new_shape == tensor_shape {
                continue;
            }
            if let Some(filter) = filter && !filter(new_shape.clone()) {
                continue;
            }
            result.push(new_shape.clone());
        }
    } else {
        for rest in gen_tile_sizes(tensor_shape[1..].to_vec(), None, false)? {
            for d in dim_range(tensor_shape[0], true)? {
                let mut new_shape = vec![d];
                new_shape.extend(rest.clone());
                if drop_given && new_shape == tensor_shape {
                    continue;
                }
                if let Some(filter) = filter && !filter(new_shape.clone()) {
                    continue;
                }
                result.push(new_shape.clone());
            }
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dim_range() {
        // Common cases
        assert_eq!(dim_range(0, true).unwrap(), vec![]);
        assert_eq!(dim_range(0, false).unwrap(), vec![]);
        assert_eq!(dim_range(1, false).unwrap(), vec![]);
        assert_eq!(dim_range(2, false).unwrap(), vec![1]);

        // POWERS_OF_TWO
        assert_eq!(dim_range(1, true).unwrap(), vec![1]);
        assert_eq!(dim_range(2, true).unwrap(), vec![1, 2]);
        assert_eq!(dim_range(3, true).unwrap(), vec![1, 2, 3]);
        assert_eq!(dim_range(4, true).unwrap(), vec![1, 2, 4]);
        assert_eq!(dim_range(5, true).unwrap(), vec![1, 2, 4, 5]);
        assert_eq!(dim_range(3, false).unwrap(), vec![1, 2]);
        assert_eq!(dim_range(4, false).unwrap(), vec![1, 2]);
        assert_eq!(dim_range(5, false).unwrap(), vec![1, 2, 4]);
    }
}
