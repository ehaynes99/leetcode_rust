pub struct Solution;
use crate::util::binary_tree::TreeNode;

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Node>) -> i32 {
        let mut result = 0;
        Self::check(&root, &mut result);
        result
    }

    fn check(node: &Option<Node>, result: &mut i32) -> i32 {
        match node {
            Some(node) => {
                let node_ref = node.borrow();
                let left_depth = Self::check(&node_ref.left, result);
                let right_depth = Self::check(&node_ref.right, result);

                *result = max(*result, left_depth + right_depth);
                max(left_depth, right_depth) + 1
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::create_tree;

    use super::*;

    #[test]
    fn test1() {
        //      1
        //     / \
        //    2  3
        //   / \
        //  4  5
        let root = create_tree(vec![1, 2, 3, 4, 5]);
        assert_eq!(3, Solution::diameter_of_binary_tree(root));
    }

    #[test]
    fn test2() {
        let root = create_tree(vec![1, 2]);
        assert_eq!(1, Solution::diameter_of_binary_tree(root));
    }

    #[test]
    fn test3() {
        let root = create_tree(vec![
            4, 7, 3, -1, -1, 9, 3, 9, 7, 4, -1, 6, -1, 6, 6, -1, -1, 0, 6, 5, -1, 9, -1, -1, 1, 4,
            -1, -1, -1, 2,
        ]);
        assert_eq!(8, Solution::diameter_of_binary_tree(root));
    }
}
