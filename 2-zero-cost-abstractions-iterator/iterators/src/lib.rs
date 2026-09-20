//! Zero-cost abstractions with iterators vs manual loops.
//! In release, these variants compile to very similar code thanks to monomorphization and inlining.

/// Manual loop: sum of squares of even numbers in [0, n).
pub fn sum_even_squares_loop(n: u64) -> u128 {
    let mut acc: u128 = 0;
    let mut i = 0_u64;
    while i < n {
        if i % 2 == 0 {
            let x = i as u128;
            acc += x * x;
        }
        i += 1;
    }
    acc
}

/// Iterator chain: filter/map/fold with static dispatch and no allocations.
pub fn sum_even_squares_iter(n: u64) -> u128 {
    (0..n)
        .filter(|&x| x % 2 == 0)
        .map(|x| {
            let x = x as u128;
            x * x
        })
        .fold(0_u128, |acc, v| acc + v)
}

/// Iterator chain using step_by to skip odds.
pub fn sum_even_squares_step(n: u64) -> u128 {
    (0..n)
        .step_by(2)
        .map(|x| {
            let x = x as u128;
            x * x
        })
        .fold(0_u128, |acc, v| acc + v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn same_results() {
        for &n in &[0, 1, 2, 3, 10, 1_000] {
            let a = sum_even_squares_loop(n);
            let b = sum_even_squares_iter(n);
            let c = sum_even_squares_step(n);
            assert_eq!(a, b);
            assert_eq!(a, c);
        }
    }

    #[test]
    fn niche_optimization_reminder() {
        // Option<&u8> takes the same space as &u8 on 64-bit targets
        assert_eq!(mem::size_of::<&u8>(), mem::size_of::<Option<&u8>>());
    }
}
