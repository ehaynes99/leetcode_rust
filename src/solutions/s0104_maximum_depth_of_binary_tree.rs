pub struct Solution;
use crate::util::binary_tree::TreeNode;

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn max_depth(root: Option<Node>) -> i32 {
        Self::check(&root)
    }

    fn check(node: &Option<Node>) -> i32 {
        match node {
            Some(node) => {
                let node_ref = node.borrow();
                let left_depth = Self::check(&node_ref.left);
                let right_depth = Self::check(&node_ref.right);

                max(left_depth, right_depth) + 1
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::{from_array, NULL};

    use super::*;

    #[test]
    fn test1() {
        let root = from_array(&[3, 9, 20, NULL, NULL, 15, 7]);
        assert_eq!(3, Solution::max_depth(root));
    }

    #[test]
    fn test2() {
        let root = from_array(&[1, NULL, 2]);
        assert_eq!(2, Solution::max_depth(root));
    }
}
