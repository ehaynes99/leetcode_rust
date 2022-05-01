use std::convert::TryFrom;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut min = 0;
        let mut max = nums.len();

        while min <= max {
            let mid = (min + max) / 2;
            let value = nums[mid];

            if value == target {
                return i32::try_from(mid).expect("Could not convert usize to i32");
            } else if value < target {
                min = mid + 1;
                if min == nums.len() {
                    break;
                }
            } else {
                if mid == 0 {
                    break;
                }
                max = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(4, Solution::search(nums, 9));
    }

    #[test]
    fn test2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(-1, Solution::search(nums, 2));
    }

    #[test]
    fn test3() {
        let nums = vec![5];
        assert_eq!(-1, Solution::search(nums, -5));
    }

    #[test]
    fn test4() {
        let nums = vec![-1,0,3,5,9,12];
        assert_eq!(-1, Solution::search(nums, 13));
    }
}
