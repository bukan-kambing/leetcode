// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
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
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let (left, right, val) = (node.left.clone(), node.right.clone(), node.val);

            if left.is_none() && right.is_none() && val == target_sum {
                return true;
            }

            target_sum -= val;

            return Self::has_path_sum(left, target_sum) || Self::has_path_sum(right, target_sum);
        }

        false
    }
}
