pub struct Solution;
use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut min = 10001;
        for price in prices {
            min = cmp::min(price, min);
            result = cmp::max(result, price - min);
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let result = Solution::max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(result, 0);
    }
}
