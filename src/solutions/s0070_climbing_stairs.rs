pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;

        for _ in 0..n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn test2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn test3() {
        assert_eq!(5, Solution::climb_stairs(4));
    }
}
