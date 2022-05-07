use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;
type MaybeNode = Option<Node>;

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

pub fn create_tree(values: Vec<i32>) -> MaybeNode {
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0])));
    let mut nodes = vec![root.clone()];

    for index in 1..values.len() {
        let value = values[index];
        if value >= 0 {
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

pub struct DepthFirstIter(VecDeque<Node>);

impl DepthFirstIter {
    pub fn new(root: &MaybeNode) -> Self {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node.clone());
        }
        Self(queue)
    }
}

impl Iterator for DepthFirstIter {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front().map(|node| {
            let node_ref = node.borrow();
            [&node_ref.left, &node_ref.right].into_iter().flatten().for_each(|node| {
                self.0.push_back(node.clone());
            });
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
