use std::cell::RefCell;
use std::rc::Rc;

use crate::util::binary_tree::TreeNode;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: MaybeNode,
        p: MaybeNode,
        q: MaybeNode,
    ) -> MaybeNode {
        if p == root || q == root {
            return root;
        }
        match (root, p, q) {
            (Some(root_ref), Some(p_ref), Some(q_ref)) => {
                let root_val = root_ref.borrow();
                let p_val = p_ref.borrow();
                let q_val = q_ref.borrow();

                let p_next = if p_val.val > root_val.val {
                    root_val.right.clone()
                } else {
                    root_val.left.clone()
                };

                let q_next = if q_val.val > root_val.val {
                    root_val.right.clone()
                } else {
                    root_val.left.clone()
                };

                if p_next == q_next {
                    Self::lowest_common_ancestor(p_next, Some(p_ref.clone()), Some(q_ref.clone()))
                } else {
                    Some(root_ref.clone())
                }
            }
            _ => panic!("Could not solve!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
