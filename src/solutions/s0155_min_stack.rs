use std::cmp::min;

pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let element = match self.stack.last() {
            Some((_, prev_min)) => {
                (val, min(val, *prev_min))
            }
            _ => (val, val)
        };
        self.stack.push(element);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(-3, stack.get_min());
        stack.pop();
        assert_eq!(0, stack.top());
        assert_eq!(-2, stack.get_min());
    }
}
