pub struct Solution;

impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut down = vec![1; len];

        for i in 1..len {
            down[i] = nums[len - i] * down[i - 1];
        }

        let mut up = 1;

        for i in 0..len {
            let result = up * down[len - i - 1];
            up *= nums[i];
            nums[i] = result;
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        assert_eq!(expected, Solution::product_except_self(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        assert_eq!(expected, Solution::product_except_self(nums));
    }
}
