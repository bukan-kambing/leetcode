// Definition for a :inary tree node.
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
use std::cell::RefCell; // Allow mutable or immutable borrowws at runtime
use std::rc::Rc; // Enable multiple owners of the same data
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Solution::search_leaf(&root, &mut sum);
        sum

    }

    fn leaf_check(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => {
                let node = root.borrow();
                if node.left.is_none() && node.right.is_none() {
                    return true;
                }
            }
            None => return false,
        }
        return false;
    }
    fn search_leaf(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        // Matching root enums if it's Some (containing value) or None
        match root {
            // If it has a value
            Some(root) => {
                let node = root.borrow();
                if Solution::leaf_check(&node.left){
                    let left_node = node.left.as_ref();
                    let value = left_node.unwrap().borrow().val;
                    *sum += value;
                }
                Solution::search_leaf(&node.left, sum);
                Solution::search_leaf(&node.right, sum);
            }
            // If it is None
            None => {}
        }
    }
}
