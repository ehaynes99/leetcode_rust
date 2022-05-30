use std::cell::RefCell;
use std::rc::Rc;

pub type Node = Rc<RefCell<TreeNode>>;
pub type MaybeNode = Option<Node>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: MaybeNode,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
