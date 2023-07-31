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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::rec(&root, i64::MIN, i64::MAX);
    }

    pub fn rec(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let val = node.val as i64;

                return val > min
                    && val < max
                    && Self::rec(&node.left, min, val)
                    && Self::rec(&node.right, val, max);
            }
            _ => return true,
        }
    }
}
