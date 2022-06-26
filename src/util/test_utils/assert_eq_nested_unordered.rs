use std::fmt::Debug;

/// TODO: don't really want to take ownership here...
pub fn assert_eq_nested_unordered<T: Ord + Debug>(expected: Vec<Vec<T>>, actual: Vec<Vec<T>>) {
    let mut expected = expected
        .into_iter()
        .map(|mut vec| {
            vec.sort_unstable();
            vec
        })
        .collect::<Vec<_>>();
    expected.sort_unstable();
    let mut actual = actual
        .into_iter()
        .map(|mut vec| {
            vec.sort_unstable();
            vec
        })
        .collect::<Vec<_>>();
    actual.sort_unstable();
    assert_eq!(expected, actual);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outer_values_different_order() {
        let expected = vec![
            vec![1, 2, 3], //
            vec![2, 3, 4],
        ];
        let actual = vec![
            vec![2, 3, 4], //
            vec![1, 2, 3],
        ];
        assert_eq_nested_unordered(expected, actual);
    }

    #[test]
    fn inner_values_different_order() {
        let expected = vec![
            vec![1, 2, 3], //
            vec![2, 3, 4],
        ];
        let actual = vec![
            vec![3, 2, 1], //
            vec![4, 3, 2],
        ];
        assert_eq_nested_unordered(expected, actual);
    }

    #[test]
    fn both_different_order() {
        let expected = vec![
            vec![1, 2, 3], //
            vec![2, 3, 4],
        ];
        let actual = vec![
            vec![4, 3, 2], //
            vec![3, 2, 1],
        ];
        assert_eq_nested_unordered(expected, actual);
    }

    #[test]
    fn directly_equal() {
        let expected = vec![
            vec![1, 2, 3], //
            vec![2, 3, 4],
            ];
            let actual = vec![
            vec![1, 2, 3], //
            vec![2, 3, 4],
        ];
        assert_eq_nested_unordered(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_not_eq() {
        let expected = vec![
            vec![1, 2, 3], //
            vec![2, 3, 4],
        ];
        let actual = vec![
            vec![1, 2, 3], //
            vec![4, 5, 6],
        ];
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq_nested_unordered(expected, actual);
    }
}