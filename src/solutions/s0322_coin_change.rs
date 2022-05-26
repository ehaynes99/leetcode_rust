pub struct Solution;

use std::cmp;

const UNKNOWN: i32 = i32::MAX;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // safe because description states coins and amounts are positive
        let amount = amount as usize;
        let mut results = vec![UNKNOWN; amount + 1];
        results[0] = 0;

        for amount in 1..amount + 1 {
            for coin in coins.iter().map(|coin| *coin as usize).filter(|coin| coin <= &amount) {
                let other = results[amount - coin];
                if other != UNKNOWN {
                    let curr_val = &mut results[amount];
                    *curr_val = cmp::min(*curr_val, other + 1);
                }
            }
        }

        match results[amount] {
            UNKNOWN => -1,
            value => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(3, Solution::coin_change(coins, amount));
    }

    #[test]
    fn test2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(-1, Solution::coin_change(coins, amount));
    }

    #[test]
    fn test3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(0, Solution::coin_change(coins, amount));
    }

    #[test]
    fn test4() {
        let coins = vec![186, 419, 83, 408];
        let amount = 6249;
        assert_eq!(20, Solution::coin_change(coins, amount));
    }
}
