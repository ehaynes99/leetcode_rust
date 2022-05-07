pub struct Solution;

const SIZE: usize = 'z' as usize - 'A' as usize + 1;
const OFFSET: usize = 'A' as usize;

impl Solution {
    pub fn longest_palindrome(str: String) -> i32 {
        let get_index = |c: char| {
            c as usize - OFFSET
        };
        let mut counts = [0i32; SIZE];
        let mut result = 0;
        for index in str.chars().map(&get_index) {
            counts[index] += 1;
            if counts[index] % 2 == 0 {
                result += 2;
            }
        }

        if counts.into_iter().any(|count| count % 2 == 1) {
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "abccccdd".to_string();
        assert_eq!(7, Solution::longest_palindrome(s))
    }

    #[test]
    fn test2() {
        let s = "a".to_string();
        assert_eq!(1, Solution::longest_palindrome(s))
    }

    #[test]
    fn test3() {
        let s = "bb".to_string();
        assert_eq!(2, Solution::longest_palindrome(s))
    }

    #[test]
    fn test4() {
        let s = "AAAAAA".to_string();
        assert_eq!(6, Solution::longest_palindrome(s))
    }

    #[test]
    fn test5() {
        let s = "aaaAAA".to_string();
        assert_eq!(5, Solution::longest_palindrome(s))
    }
}
