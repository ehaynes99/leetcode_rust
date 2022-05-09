pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let target = nums.len() / 2;
        let mut counts = HashMap::with_capacity(nums.len());

        for num in nums {
            let entry = counts.entry(num).or_insert(0);
            *entry += 1;
            if *entry > target {
                return num;
            }
        }
        panic!("Could not solve!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        assert_eq!(3, Solution::majority_element(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(2, Solution::majority_element(nums));
    }
}
