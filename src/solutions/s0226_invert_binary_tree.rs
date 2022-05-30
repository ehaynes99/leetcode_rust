pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

use crate::util::binary_tree::TreeNode;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn invert_tree(node: MaybeNode) -> Option<Rc<RefCell<TreeNode>>> {
        node.map(|rc| {
            {
                let mut value = rc.borrow_mut();

                let new_right = Self::invert_tree(value.left.take());
                let new_left = Self::invert_tree(value.right.take());
                value.left = new_left;
                value.right = new_right;
            }
            rc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::binary_tree::{from_array,level_order_values};

    fn solve(root: MaybeNode) -> Vec<i32> {
        level_order_values(Solution::invert_tree(root))
    }

    #[test]
    fn test1() {
        let root = from_array(&[4, 2, 7, 1, 3, 6, 9]);
        let expected = vec![4, 7, 2, 9, 6, 3, 1];
        assert_eq!(expected, solve(root));
    }

    #[test]
    fn test2() {
        let root = from_array(&[2, 1, 3]);
        let expected = vec![2, 3, 1];
        assert_eq!(expected, solve(root));
    }

    #[test]
    fn test3() {
        let root = from_array(&[]);
        let expected = Vec::<i32>::new();
        assert_eq!(expected, solve(root));
    }
}
