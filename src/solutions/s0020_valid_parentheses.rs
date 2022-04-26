pub struct Solution;

impl Solution {
    pub fn is_valid(string: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in string.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                ')' => {
                    match stack.last() {
                        Some(&pair) => {
                            if pair != '(' { return false }
                            stack.pop();
                        },
                        _ => return false,
                    }
                },
                ']' => {
                    match stack.last() {
                        Some(&pair) => {
                            if pair != '[' { return false }
                            stack.pop();
                        },
                        _ => return false,
                    }
                },
                '}' => {
                    match stack.last() {
                        Some(&pair) => {
                            if pair != '{' { return false }
                            stack.pop();
                        },
                        _ => return false,
                    }
                },
                _ => panic!("Invalid char: {}", c),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }
}
