use std::collections::VecDeque;

use super::{children, MaybeNode, Node};

pub struct LevelOrderIterator {
    queue: VecDeque<Node>,
}

impl LevelOrderIterator {
    pub fn new(root: Node) -> Self {
        Self {
            queue: VecDeque::from(vec![root]),
        }
    }
}

impl Iterator for LevelOrderIterator {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|node| {
            for child in children(&node) {
                self.queue.push_back(child.clone());
            }
            node
        })
    }
}

pub fn level_order_values(root: MaybeNode) -> Vec<i32> {
    if root.is_none() {
        return Vec::new();
    }
    let iter = LevelOrderIterator::new(root.unwrap());
    iter.map(|node| node.borrow().val).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::super::{from_array, NULL};
    use super::*;

    #[test]
    fn traverses_nodes_from_top_to_bottom() {
        let root = from_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, NULL, NULL, 13, 14]);
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14];
        let result = level_order_values(root);

        assert_eq!(expected, result);
    }
}
