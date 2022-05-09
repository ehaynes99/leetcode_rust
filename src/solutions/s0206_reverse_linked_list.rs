use crate::util::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = head;
        let mut result = None;

        while let Some(mut head) = tail {
            tail = head.next.take();

            head.next = result;
            result = Some(head);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::linked_list::{list_from_vec, list_to_vec};

    use super::*;

    #[test]
    fn test1() {
        let list = list_from_vec(vec![1,2,3,4,5]);
        let result = Solution::reverse_list(list);
        assert_eq!(vec![5,4,3,2,1], list_to_vec(result));
    }

    #[test]
    fn test2() {
        let list = list_from_vec(vec![]);
        let result = Solution::reverse_list(list);
        assert_eq!(None, result);
    }
}
