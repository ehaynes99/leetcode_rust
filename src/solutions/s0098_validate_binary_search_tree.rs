pub struct Solution;

use crate::util::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let mut stack = vec![(root.clone(), (i64::MIN, i64::MAX))];

        while !stack.is_empty() {
            let (node, bounds) = stack.pop().unwrap();
            let node_ref = node.borrow();
            let val = node_ref.val as i64;

            if node != root && (val <= bounds.0 || val >= bounds.1) {
                return false;
            }

            if let Some(left) = &node_ref.left {
                stack.push((left.clone(), (bounds.0, val)));
            }
            if let Some(right) = &node_ref.right {
                stack.push((right.clone(), (val, bounds.1)));
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::{create_node, from_array, NULL, set_left};

    use super::*;

    #[test]
    fn test1() {
        let root = from_array(&[2, 1, 3]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test2() {
        let root = from_array(&[5, 1, 4, NULL, NULL, 3, 6]);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn test3() {
        let root = from_array(&[5, 4, 6, NULL, NULL, 3, 7]);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn test4() {
        // from_array helper uses i32::MIN as NULL, so need to do it manually here
        let mut root = create_node(-2147483648);
        set_left(&mut root, -2147483648);
        assert!(!Solution::is_valid_bst(Some(root)));
    }

    #[test]
    fn test5() {
        let root = from_array(&[-2147483648, NULL, 2147483647]);
        assert!(Solution::is_valid_bst(root));
    }
}
