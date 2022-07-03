// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = Solution::check_depth(&root);
        depth
    }

    fn check_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let node = root.borrow();
                let mut ldepth = Solution::check_depth(&node.left);
                let mut rdepth = Solution::check_depth(&node.right);
                if ldepth > rdepth {
                    return ldepth + 1
                }
                return rdepth + 1
            }
            None => 0,
        }
    }
}
