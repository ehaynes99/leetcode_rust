pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = Vec::with_capacity(a.len() + b.len());
        let (mut a, mut b) = if a.len() > b.len() { (a, b) } else { (b, a) };

        let mut remainder = 0;

        while !b.is_empty() {
            let a_val = a.pop().unwrap_or('0');
            let b_val = b.pop().unwrap();


            let sum = remainder + a_val.to_digit(2).unwrap() + b_val.to_digit(2).unwrap();

            match sum {
                0 => {
                    result.push('0');
                    // remainder already 0
                }
                1 => {
                    result.push('1');
                    remainder = 0;
                }
                2 => {
                    result.push('0');
                    remainder = 1;
                }
                _ => {
                    result.push('1');
                    // remainder already 0
                }
            }
            if b.is_empty() && remainder == 1 {
                b.push('1');
                remainder = 0;
            }
        }

        result.reverse();
        format!("{}{}", a, result.into_iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!("100".to_string(), Solution::add_binary(a, b));
    }

    #[test]
    fn test2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!("10101".to_string(), Solution::add_binary(a, b));
    }

    #[test]
    fn test3() {
        let a = "101111".to_string();
        let b = "10".to_string();
        assert_eq!("110001".to_string(), Solution::add_binary(a, b));
    }
}
