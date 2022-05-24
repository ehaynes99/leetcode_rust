pub struct Solution;

use crate::util::binary_tree::TreeNode;

type Node = Rc<RefCell<TreeNode>>;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Node>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if let Some(root) = root {
            let mut stack = vec![vec![root]];

            loop {
                let nodes = stack.last().unwrap();
                result.push(
                    nodes
                        .iter()
                        .map(|node| node.borrow().val)
                        .collect::<Vec<_>>(),
                );

                let next = Self::next_row(nodes);
                if next.is_empty() {
                    break;
                }
                stack.push(next);
            }
        }
        result
    }

    fn next_row(nodes: &Vec<Node>) -> Vec<Node> {
        let mut result = Vec::new();

        for node in nodes {
            let node_ref = node.borrow();
            if let Some(left) = &node_ref.left {
                result.push(left.clone());
            }
            if let Some(right) = &node_ref.right {
                result.push(right.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::{from_vec, NULL};

    use super::*;

    #[test]
    fn test1() {
        let root = from_vec(vec![3, 9, 20, NULL, NULL, 15, 7]);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(expected, Solution::level_order(root));
    }

    #[test]
    fn test2() {
        let root = from_vec(vec![1]);
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::level_order(root));
    }

    #[test]
    fn test3() {
        let root = from_vec(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, Solution::level_order(root));
    }
}
