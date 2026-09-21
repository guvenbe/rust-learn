//! Zero-cost abstractions live here.

/// Iterator-based even counter (expressive, compiles down to tight loops).
pub fn count_even_numbers(numbers: &[i32]) -> usize {
    numbers.iter().filter(|&&x| x % 2 == 0).count()
}

/// Manual loop version: same semantics, similar codegen when optimized.
pub fn count_even_numbers_manual(numbers: &[i32]) -> usize {
    let mut count = 0;
    for &n in numbers {
        if n % 2 == 0 {
            count += 1;
        }
    }
    count
}

/// Generic predicate counter (monomorphized at compile time).
pub fn count_matching<T, F>(iter: impl IntoIterator<Item = T>, mut pred: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    iter.into_iter().filter(|x| pred(x)).count()
}

/// Small generic map+collect to stress monomorphization without runtime overhead.
pub fn transform<T, U, F>(iter: impl IntoIterator<Item = T>, mut f: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    iter.into_iter().map(|x| f(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even() {
        let nums = [1,2,3,4,5,6,7,8];
        assert_eq!(count_even_numbers(&nums), 4);
        assert_eq!(count_even_numbers_manual(&nums), 4);
        assert_eq!(count_matching(nums, |n| n%2==0), 4);
    }

    #[test]
    fn test_transform() {
        let out = transform([1,2,3], |x| x * 10);
        assert_eq!(out, vec![10,20,30]);
    }
}
