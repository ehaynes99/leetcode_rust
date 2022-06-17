use std::{cell::RefCell, rc::Rc};

use super::{Node, MaybeNode, TreeNode};

pub const NULL: i32 = i32::MIN;

pub fn create_node(value: i32) -> Node {
    Rc::new(RefCell::new(TreeNode::new(value)))
}

pub fn from_array(values: &[i32]) -> MaybeNode {
    from_vec(values.to_vec())
}

pub fn from_vec(values: Vec<i32>) -> MaybeNode {
    if values.is_empty() {
        return None;
    }

    let root = create_node(values[0]);
    let mut nodes = vec![root.clone()];

    for index in 1..values.len() {
        let value = values[index];
        if value != NULL {
            let node = create_node(value);
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

pub fn set_left(node: &mut Node, value: i32) {
    node.borrow_mut().left.replace(create_node(value));
}

pub fn set_right(node: &mut Node, value: i32) {
    node.borrow_mut().left.replace(create_node(value));
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