use std::cmp;

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut curr_max = nums[0];

        for i in 1..nums.len() {
            let num = nums[i];
            curr_max = cmp::max(num, curr_max + num);
            max = cmp::max(max, curr_max);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(6, Solution::max_sub_array(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![1];
        assert_eq!(1, Solution::max_sub_array(nums));
    }

    #[test]
    fn test3() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(23, Solution::max_sub_array(nums));
    }
}
