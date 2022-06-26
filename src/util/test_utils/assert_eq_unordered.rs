use std::fmt::Debug;

pub fn assert_eq_unordered<T: Ord + Debug + Clone>(expected: Vec<T>, actual: Vec<T>) {
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    let mut actual = actual.to_vec();
    actual.sort_unstable();
    assert_eq!(expected, actual);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_values() {
        let expected = vec![1, 2, 3, 4];
        let actual = vec![1, 2, 3, 4];
        assert_eq_unordered(expected, actual);
    }

    #[test]
    fn same_values_different_order() {
        let expected = vec![1, 2, 3, 4];
        let actual = vec![4, 3, 1, 2];
        assert_eq_unordered(expected, actual);
    }

    #[test]
    #[should_panic]
    fn not_equal() {
        std::panic::set_hook(Box::new(|_| {}));

        let expected = vec![1, 2, 3, 4];
        let actual = vec![2, 3, 4, 5];
        assert_eq_unordered(expected, actual);
    }

    #[test]
    fn tmp() {
        let expected = vec![2, 4, 3, 1];
        let actual = vec![2, 4, 3, 1];
        assert_eq!(expected, actual);
        assert_eq!(vec![2, 4, 3, 1], expected);
        assert_eq!(vec![2, 4, 3, 1], actual);
    }
}