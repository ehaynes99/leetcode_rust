pub struct Solution;

use std::cmp::Ordering::*;
use std::collections::HashSet;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::solve(&candidates, target).into_iter().collect()
    }

    fn solve(candidates: &[i32], target: i32) -> HashSet<Vec<i32>> {
        let mut result = HashSet::new();

        for index in 0..candidates.len() {
            let candidate = candidates[index];

            match target.cmp(&candidate) {
                Less => (),
                Equal => {
                    result.insert(vec![candidate]);
                }
                Greater => {
                    for mut vec in Self::solve(&candidates[index..], target - candidate) {
                        vec.push(candidate);
                        result.insert(vec);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_same_values(expected: Vec<Vec<i32>>, actual: Vec<Vec<i32>>) {
        let mut expected = expected.into_iter().map(|mut vec| {
            vec.sort_unstable();
            vec
        }).collect::<Vec<_>>();
        expected.sort();
        let mut actual = actual.into_iter().map(|mut vec| {
            vec.sort_unstable();
            vec
        }).collect::<Vec<_>>();
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let expected: Vec<Vec<i32>> = vec![vec![2, 2, 3], vec![7]];
        assert_same_values(expected, Solution::combination_sum(candidates, target));
    }

    #[test]
    fn test2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let expected: Vec<Vec<i32>> = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_same_values(expected, Solution::combination_sum(candidates, target));
    }

    #[test]
    fn test3() {
        let candidates = vec![2];
        let target = 1;
        let expected: Vec<Vec<i32>> = vec![];
        assert_same_values(expected, Solution::combination_sum(candidates, target));
    }
}
