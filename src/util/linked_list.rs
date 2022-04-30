#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut current = &list;
    loop {
        if let Some(node) = current {
            res.push(node.val);
            current = &node.next;
        } else {
            return res;
        }
    }
}

pub fn list_from_vec(items: Vec<i32>) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = None;
    let mut current = &mut root;
    for item in items {
        let item_box = Box::new(ListNode::new(item));
        current.replace(item_box);
        if let Some(node) = current {
            current = &mut node.next;
        }
    }

    root
}

#[macro_export]
macro_rules! linked_list {
    ( $( $x:expr ),* ) => {
        {
            let mut list: Option<Box<ListNode>> = None;
            let mut _curr = &mut list;
            $(
                let node = Box::new(ListNode::new($x));
                _curr.replace(node);

                if let Some(node) = _curr {
                    _curr = &mut node.next;
                }
            )*
            list
        }
    };
}
