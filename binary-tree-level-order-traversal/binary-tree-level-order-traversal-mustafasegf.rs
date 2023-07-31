// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::with_capacity(1_000);
        let mut ans = Vec::with_capacity(1_000);

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let mut row = Vec::new();

            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    row.push(node.borrow().val);

                    if let Some(left) = node.borrow_mut().left.take() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.borrow_mut().right.take() {
                        queue.push_back(right);
                    }
                }
            }
            ans.push(row);
        }

        return ans;
    }
}
