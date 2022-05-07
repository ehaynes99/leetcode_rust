pub struct Solution;

use crate::util::binary_tree::TreeNode;

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_balanced(root: MaybeNode) -> bool {
        let root_depth = Self::balanced_depth(&root);
        root_depth != None
    }

    fn balanced_depth(node: &MaybeNode) -> Option<i32> {
        match node {
            None => Some(0),
            Some(node) => {
                let node_ref = node.borrow();
                if let Some(left) = Self::balanced_depth(&node_ref.left) {
                    if let Some(right) = Self::balanced_depth(&node_ref.right) {
                        if (left - right).abs() < 2 {
                            return Some(1 + max(left, right));
                        }
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::create_tree;

    use super::*;

    #[test]
    fn test1() {
        //     1
        //   /  \
        //  9    20
        //      / \
        //    15  7
        let root = create_tree(vec![3, 9, 20, -1, -1, 15, 7]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    #[rustfmt::skip]
    fn test2() {
        //           1
        //         /  \
        //        2    2
        //      / \   / \
        //     3  3  N  N
        //   / \
        //  4  4
        let root = create_tree(vec![1,2,2,3,3,-1,-1,4,4]);
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn test3() {
        //           1
        //         /  \
        //        2    3
        //      / \   / \
        //     4  5  6  null
        //   /
        //  8
        let root = create_tree(vec![1, 2, 3, 4, 5, 6, -1, 8]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test4() {
        //            1
        //          /   \
        //        2     2
        //      /  \   / \
        //     3   N  N  3
        //   /  \      /  \
        //  4   N     N   4
        let root = create_tree(vec![1, 2, 2, 3, -1, -1, 3, 4, -1, -1, 4]);
        assert!(!Solution::is_balanced(root));
    }
}
