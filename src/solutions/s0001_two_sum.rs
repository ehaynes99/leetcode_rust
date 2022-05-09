pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for a in 0..nums.len() {
            if a as i32 >= target && target > 0 {
                continue;
            }
            for b in (a + 1)..nums.len() {
                if nums[a] + nums[b] == target {
                    return vec![a as i32, b as i32];
                }
            }
        }
        panic!("Couldn't solve!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test4() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 3]);
    }
}
