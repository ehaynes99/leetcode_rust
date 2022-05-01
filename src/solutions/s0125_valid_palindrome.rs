pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();

        let mut i = 0;
        while i < chars.len() {
            if chars[i] != chars.pop().unwrap() {
                return false;
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        assert!(Solution::is_palindrome(s))
    }

    #[test]
    fn test2() {
        let s = "race a car".to_string();
        assert!(!Solution::is_palindrome(s))
    }

    #[test]
    fn test3() {
        let s = " ".to_string();
        assert!(Solution::is_palindrome(s))
    }

    #[test]
    fn test4() {
        let s = "0P".to_string();
        assert!(!Solution::is_palindrome(s))
    }
}
