use itertools::Itertools;

pub fn iter_is_sorted<T: PartialOrd + Clone>(iter: impl Iterator<Item = T>) -> bool {
    iter.tuple_windows().all(|(l, r)| r >= l)
}

pub fn slice_is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}

pub fn slice_by_key_is_sorted<T, K: PartialOrd>(slice: &[T], f: impl Fn(&T) -> K) -> bool {
    slice.windows(2).all(|w| f(&w[0]) <= f(&w[1]))
}

#[cfg(test)]
mod tests {
    use super::iter_is_sorted;

    #[test]
    fn is_sorted() {
        assert!(iter_is_sorted([0; 0].iter()));
        assert!(iter_is_sorted([1].iter()));
        assert!(iter_is_sorted([1, 2].iter()));
        assert!(iter_is_sorted([1, 2, 3].iter()));
    }
}
