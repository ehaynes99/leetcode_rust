pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; };

        let mut chars: Vec<char> = x.to_string().chars().collect();
        for i in 0..chars.len() / 2 {
            if chars[i] != chars.pop().unwrap() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(Solution::is_palindrome(12321));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!Solution::is_palindrome(12345));
    }

    #[test]
    fn negative_number() {
        assert!(!Solution::is_palindrome(-12321));
    }
}
