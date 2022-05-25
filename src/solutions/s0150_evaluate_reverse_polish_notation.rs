pub struct Solution;

struct Calc {
    stack: Vec<i32>,
}

impl Calc {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn input(&mut self, str: &str) {
        match str {
            "+" => self.operate(|a, b| a + b),
            "-" => self.operate(|a, b| a - b),
            "*" => self.operate(|a, b| a * b),
            "/" => self.operate(|a, b| a / b),
            _ => self.stack.push(str.parse().unwrap())
        }
    }

    pub fn result(&mut self) -> i32 {
        if self.stack.len() != 1 {
            panic!("Invalid input! {:?}", self.stack);
        }
        self.stack.pop().unwrap()
    }

    fn operate(&mut self, operation: impl Fn(i32, i32) -> i32) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(operation(a, b));
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut calc = Calc::new();

        for token in tokens {
            calc.input(&token);
        }
        calc.result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_vec(vals: &[&str]) -> Vec<String> {
        vals.iter().map(|s| s.to_string()).collect()
    }
    #[test]
    fn test1() {
        let tokens = string_vec(&["2", "1", "+", "3", "*"]);
        assert_eq!(9, Solution::eval_rpn(tokens));
    }

    #[test]
    fn test2() {
        let tokens = string_vec(&["4", "13", "5", "/", "+"]);
        assert_eq!(6, Solution::eval_rpn(tokens));
    }

    #[test]
    fn test3() {
        let tokens = string_vec(&[
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]);
        assert_eq!(22, Solution::eval_rpn(tokens));
    }
}
