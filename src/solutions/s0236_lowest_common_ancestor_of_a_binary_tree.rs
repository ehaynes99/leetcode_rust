pub struct Solution;
use crate::util::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
pub type Node = Rc<RefCell<TreeNode>>;
pub type MaybeNode = Option<Node>;

impl Solution {
    pub fn lowest_common_ancestor(root: MaybeNode, p: MaybeNode, q: MaybeNode) -> MaybeNode {
        let root = root.unwrap();
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        let mut paths = Vec::new();
        Self::dfs(&root, p, q, &mut Vec::new(), &mut paths);

        let mut ancestor = 0;
        for (i, val) in paths[0].iter().enumerate() {
            if paths[1].len() <= i {
                break;
            }
            if paths[1][i] == *val {
                ancestor = *val;
            }
        }

        Some(Rc::new(RefCell::new(TreeNode::new(ancestor))))
    }

    fn dfs(node: &Node, p: i32, q: i32, curr_path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        let node_ref = node.borrow();
        let val = node_ref.val;
        curr_path.push(val);
        if val == p || val == q {
            paths.push(curr_path.clone());
            if paths.len() == 2 {
                return;
            }
        }
        if let Some(left) = &node_ref.left {
            Self::dfs(left, p, q, curr_path, paths);
        }
        if let Some(right) = &node_ref.right {
            Self::dfs(right, p, q, curr_path, paths);
        }
        curr_path.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::{create_node, from_array, NULL};

    use super::*;

    fn get_solution(values: &[i32], p: i32, q: i32) -> i32 {
        let root = from_array(values);

        let p = Some(create_node(p));
        let q = Some(create_node(q));

        Solution::lowest_common_ancestor(root, p, q)
            .unwrap()
            .borrow()
            .val
    }

    #[test]
    fn test1() {
        let values = &[3, 5, 1, 6, 2, 0, 8, NULL, NULL, 7, 4];
        let p = 5;
        let q = 1;
        assert_eq!(3, get_solution(values, p, q));
    }

    #[test]
    fn test2() {
        let values = &[3, 5, 1, 6, 2, 0, 8, NULL, NULL, 7, 4];
        let p = 5;
        let q = 4;
        assert_eq!(5, get_solution(values, p, q));
    }

    #[test]
    fn test3() {
        let values = &[1, 2];
        let p = 1;
        let q = 2;
        assert_eq!(1, get_solution(values, p, q));
    }

    #[test]
    fn test4() {
        let values = &[1, 2, 3, NULL, 4];
        let p = 4;
        let q = 3;
        assert_eq!(1, get_solution(values, p, q));
    }
}
