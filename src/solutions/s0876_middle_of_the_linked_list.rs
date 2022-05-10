pub struct Solution;
use crate::util::linked_list::ListNode;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = head.as_ref();
        let mut result = head.as_ref();
        let mut index = 0;

        while tail.is_some() {
            index += 1;
            tail = tail.unwrap().next.as_ref();

            if index % 2 == 0 {
                result = result.unwrap().next.as_ref();
            }
        }
        result.cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::linked_list::{list_from_vec, list_to_vec};

    use super::*;

    #[test]
    fn test1() {
        let list = list_from_vec(vec![1, 2, 3, 4, 5]);
        let result = Solution::middle_node(list);
        assert_eq!(vec![3, 4, 5], list_to_vec(result));
    }

    #[test]
    fn test2() {
        let list = list_from_vec(vec![1, 2, 3, 4, 5, 6]);
        let result = Solution::middle_node(list);
        assert_eq!(vec![4, 5, 6], list_to_vec(result));
    }
}
