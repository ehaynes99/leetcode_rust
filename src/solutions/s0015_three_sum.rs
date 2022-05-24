pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums.len() < 3 {
            return result;
        }

        nums.sort_unstable();
        let indexes = Self::max_indexes(&nums);

        for i in 0..nums.len() - 2 {
            let a = nums[i];
            if i > 0 && nums[i - 1] == a {
                continue;
            }

            for j in i + 1..nums.len() - 1 {
                let b = nums[j];
                if j > i + 1 && nums[j - 1] == b {
                    continue;
                }

                let value = 0 - (a + b);
                if let Some(max_index) = indexes.get(&value) {
                    if max_index > &j {
                        result.push(vec![a, b, value]);
                    }
                }
            }
        }

        result
    }

    fn max_indexes(nums: &Vec<i32>) -> HashMap<i32, usize> {
        let mut result = HashMap::<i32, usize>::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            let entry = result.entry(*num).or_default();
            *entry = i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(expected, Solution::three_sum(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, Solution::three_sum(nums));
    }

    #[test]
    fn test3() {
        let nums = vec![0];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, Solution::three_sum(nums));
    }
}
