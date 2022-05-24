pub struct Solution;

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut letters = HashMap::<char, usize>::with_capacity(s.len());
        let mut max = 0;
        let mut start = 0;

        for (i, c) in s.char_indices() {
            if let Some(index) = letters.insert(c, i) {
                start = cmp::max(start, index + 1);
            }
            max = cmp::max(max, i - start + 1);

        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "abcabcbb".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn test2() {
        let s = "bbbbb".to_string();
        assert_eq!(1, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn test3() {
        let s = "pwwkew".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn test4() {
        let s = "aab".to_string();
        assert_eq!(2, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn test5() {
        let s = "".to_string();
        assert_eq!(0, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn test6() {
        let s = "a".to_string();
        assert_eq!(1, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn test7() {
        let s = "ab".to_string();
        assert_eq!(2, Solution::length_of_longest_substring(s));
    }
    #[test]
    fn test8() {
        let s = "abba".to_string();
        assert_eq!(2, Solution::length_of_longest_substring(s));
    }
}
