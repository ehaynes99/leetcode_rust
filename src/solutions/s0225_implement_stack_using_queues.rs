use std::collections::VecDeque;

#[derive(Default)]
pub struct MyStack {
    a: VecDeque<i32>,
    b: VecDeque<i32>,
}

impl MyStack {

    pub fn new() -> Self {
        Self { a: VecDeque::new(), b: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        while !self.b.is_empty() {
            self.a.push_back(self.b.pop_front().unwrap());
        }
        self.b.push_back(x);
        while !self.a.is_empty() {
            self.b.push_back(self.a.pop_front().unwrap());
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.b.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        self.b[0]
    }

    pub fn empty(&self) -> bool {
        self.b.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(2, stack.top());
        assert_eq!(2, stack.pop());
        assert!(!stack.empty());
        assert_eq!(1, stack.top());
        assert_eq!(1, stack.pop());
        assert!(stack.empty());
    }
}
