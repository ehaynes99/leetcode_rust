pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let indexes = Self::max_indexes(&nums);

        for i in 0..nums.len() - 1 {
            let value = target - nums[i];
            if let Some(max_index) = indexes.get(&value) {
                if max_index > &i {
                    return vec![i as i32, *max_index as i32];
                }
            }
        }
        panic!("Couldn't solve!")
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
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(vec![0, 1], Solution::two_sum(nums, target));
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(vec![1, 2], Solution::two_sum(nums, target));
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(vec![0, 1], Solution::two_sum(nums, target));
    }

    #[test]
    fn test4() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        assert_eq!(vec![0, 3], Solution::two_sum(nums, target));
    }
}
