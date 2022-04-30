pub struct Solution;

use crate::util::linked_list::ListNode;

impl Solution {
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

    fn test_values(list1: Vec<i32>, list2: Vec<i32>, expected: Vec<i32>) {
        let result = Solution::merge_two_lists(list_from_vec(list1), list_from_vec(list2));
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test1() {
        test_values(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test2() {
        test_values(vec![], vec![], vec![]);
    }

    #[test]
    fn test3() {
        test_values(vec![], vec![0], vec![0]);
    }
}
