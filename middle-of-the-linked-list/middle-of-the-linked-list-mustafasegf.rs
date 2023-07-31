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
impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut n = 0;

        while let Some(next_node) = fast.and_then(|n| n.next.as_ref()) {
            fast = next_node.next.as_ref();
            n += 1;
        }
        drop(fast);

        for _ in 0..n {
            head = head.unwrap().next;
        }
        head
    }
}
