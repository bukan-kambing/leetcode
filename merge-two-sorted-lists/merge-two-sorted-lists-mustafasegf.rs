// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::mem::swap;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ptr = &mut list1;
        while list2.is_some() {
            if ptr.is_none() || list2.as_ref()?.val < ptr.as_ref()?.val {
                swap(ptr, &mut list2);
            }
            ptr = &mut ptr.as_mut()?.next;
        }
        return list1;
    }
}
