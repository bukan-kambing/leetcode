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

use std::mem;
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut fast = head.as_ref();
        let mut n = 0;

        // count middle point
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref().and_then(|n| n.next.as_ref());
            n += 1;
        }

        drop(fast);

        // go to middle point
        let mut mid = {
            let mut tmp = &mut head;
            for _ in 0..n {
                tmp = &mut tmp.as_mut().unwrap().next;
            }
            tmp.take()
        };

        // reverse middle point
        let mut prev = None;
        while let Some(mut node) = mid {
            mid = mem::replace(&mut node.next, prev);
            prev = Some(node);
        }

        let mut rev_ptr = &mut prev;
        let mut head_ptr = &mut head;

        // loop to check palindrome
        for _ in 0..n {
            if rev_ptr.as_ref().unwrap().val != head_ptr.as_ref().unwrap().val {
                return false;
            }
            rev_ptr = &mut rev_ptr.as_mut().unwrap().next;
            head_ptr = &mut head_ptr.as_mut().unwrap().next;
        }

        return true;
    }
}
