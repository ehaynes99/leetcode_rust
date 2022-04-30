use crate::util::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut reduced = Vec::new();

        while !lists.is_empty() {
            reduced.push(match (lists.pop(), lists.pop()) {
                (Some(l), None) | (None, Some(l)) => l,
                (Some(a), Some(b)) => Self::merge_two_lists(a, b),
                _ => None,
            });
        }

        match reduced.len() {
            0 => None,
            1 => reduced.pop().unwrap(),
            _ => Self::merge_k_lists(reduced),
        }
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(mut a), Some(mut b)) => {
                if a.val < b.val {
                    a.next = Self::merge_two_lists(a.next, Some(b));
                    Some(a)
                } else {
                    b.next = Self::merge_two_lists(b.next, Some(a));
                    Some(b)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::linked_list::{list_from_vec, list_to_vec};

    use super::*;

    #[test]
    fn test1() {
        let result = list_to_vec(Solution::merge_k_lists(vec![
            list_from_vec(vec![1, 4, 5]),
            list_from_vec(vec![1, 3, 4]),
            list_from_vec(vec![2, 6]),
        ]));
        assert_eq!(result, vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test2() {
        let result = list_to_vec(Solution::merge_k_lists(vec![]));
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = list_to_vec(Solution::merge_k_lists(vec![None, list_from_vec(vec![0])]));
        assert_eq!(result, vec![0]);
    }
}
