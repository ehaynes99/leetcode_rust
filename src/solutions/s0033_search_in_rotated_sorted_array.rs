pub struct Solution;

use std::cmp::Ordering::*;

struct SubList<'a> {
    offset: usize,
    values: &'a [i32],
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for list in Self::split_list(&nums) {
            let result = Self::binary_search(list.values, target);
            if result >= 0 {
                return result + list.offset as i32;
            }
        }
        -1
    }

    fn binary_search(nums: &[i32], target: i32) -> i32 {
        let mut min = 0;
        let mut max = nums.len();

        while min < max {
            let mid = (min + max) / 2;

            match nums[mid].cmp(&target) {
                Equal => return mid as i32,
                Less => min = mid + 1,
                Greater => max = mid,
            }
        }

        -1
    }

    fn split_list(nums: &[i32]) -> Vec<SubList> {
        for index in 1..nums.len() {
            if nums[index] < nums[index - 1] {
                return vec![
                    SubList { offset: 0, values: &nums[..index] },
                    SubList { offset: index, values: &nums[index..] },
                ];
            }
        }
        vec![SubList { offset: 0, values: nums }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(4, Solution::search(nums, target));
    }

    #[test]
    fn test2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(-1, Solution::search(nums, target));
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(-1, Solution::search(nums, target));
    }

    // =======================

    #[test]
    fn test4() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let lists = Solution::split_list(&nums);
        assert_eq!(2, lists.len());
        assert_eq!(vec![4, 5, 6, 7], lists[0].values);
        assert_eq!(0, lists[0].offset);
        assert_eq!(vec![0, 1, 2], lists[1].values);
        assert_eq!(4, lists[1].offset);
    }

    #[test]
    fn test5() {
        let nums = vec![0, 1, 2, 4, 5, 6, 7];
        let lists = Solution::split_list(&nums);
        assert_eq!(1, lists.len());
        assert_eq!(nums, lists[0].values);
        assert_eq!(0, lists[0].offset);
    }
}
