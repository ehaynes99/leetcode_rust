use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;
type MaybeNode = Option<Node>;
pub const NULL: i32 = i32::MIN;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: MaybeNode,
    pub right: MaybeNode,
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

pub fn from_array(values: &[i32]) -> MaybeNode {
    from_vec(values.to_vec())
}

pub fn from_vec(values: Vec<i32>) -> MaybeNode {
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0])));
    let mut nodes = vec![root.clone()];

    for index in 1..values.len() {
        let value = values[index];
        if value != NULL {
            let node = Rc::new(RefCell::new(TreeNode::new(value)));
            nodes.push(node.clone());
            let parent = &nodes[(index - 1) / 2];

            if index % 2 == 1 {
                parent.borrow_mut().left.replace(node.clone());
            } else {
                parent.borrow_mut().right.replace(node.clone());
            }
        }
    }

    Some(root)
}

pub fn children(node: &Node) -> Vec<Node> {
    let mut result = Vec::new();
    let node_ref = node.borrow();
    if let Some(left) = &node_ref.left {
        result.push(left.clone());
    }
    if let Some(right) = &node_ref.right {
        result.push(right.clone());
    }
    result
}

pub struct BreadthFirstIter(VecDeque<Node>);

impl BreadthFirstIter {
    pub fn new(root: &MaybeNode) -> Self {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node.clone());
        }
        Self(queue)
    }
}

impl Iterator for BreadthFirstIter {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front().map(|node| {
            for child in children(&node) {
                self.0.push_back(child.clone());
            }
            node.clone()
        })
    }
}

pub fn print_row(row: &Vec<Node>) -> String {
    let values = row
        .into_iter()
        .map(|node| node.borrow().val)
        .collect::<Vec<_>>();
    format!("***** row: {:?}", values)
}
