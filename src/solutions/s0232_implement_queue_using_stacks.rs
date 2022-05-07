pub struct MyQueue {
    a: Vec<i32>,
    b: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            a: vec![],
            b: vec![],
        }
    }

    pub fn push(&mut self, x: i32) {
        while !self.b.is_empty() {
            self.a.push(self.b.pop().unwrap());
        }
        self.b.push(x);
        while !self.a.is_empty() {
            self.b.push(self.a.pop().unwrap());
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.b.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.b.last().unwrap()
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
        let mut queue = MyQueue::new();
        queue.push(1);
        assert_eq!(1, queue.peek());
        assert_eq!(1, queue.pop());
        assert!(queue.empty());
    }
}
