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
use std::cmp::Ordering;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(mut root: Node, p: Node, q: Node) -> Node {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        while let Some(mut node) = root {
            let mut node = node.borrow_mut();

            match (p_val.cmp(&node.val), q_val.cmp(&node.val)) {
                (Ordering::Less, Ordering::Less) => root = node.left.take(),
                (Ordering::Greater, Ordering::Greater) => root = node.right.take(),
                _ => return Some(Rc::new(RefCell::new(TreeNode::new(node.val)))),
            }
        }
        None
    }
}
