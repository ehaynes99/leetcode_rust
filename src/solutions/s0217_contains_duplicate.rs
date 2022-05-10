pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut uniq = HashSet::new();

        for num in nums {
            if !uniq.insert(num) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 1];
        assert!(Solution::contains_duplicate(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4];
        assert!(!Solution::contains_duplicate(nums));
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(Solution::contains_duplicate(nums));
    }
}
