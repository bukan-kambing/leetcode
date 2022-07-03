// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// //
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
//
use std::cell::RefCell; // Allow mutable or immutable borrowws at runtime
use std::rc::Rc; // Enable multiple owners of the same data
impl Solution {
    // the root data value mean that it is an enum containing None or Some
    //That can have multiple owners (thanks to RC)
    //And can be compiled in running time
    //That contain a stuct known as TreeNode
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 { 
        let mut sum = 0; // variable to hold the sum
        Solution::summary(&root, 0, &mut sum); // method to get the sum
        sum // actually return the sum
    }

    fn summary(root: &Option<Rc<RefCell<TreeNode>>>, value: i32, sum: &mut i32) {
        // Matching root enums if it's Some (containing value) or None
        match root {
            // If it has a value
            Some(root) => {
                let node = root.borrow(); // Getting the immutable reference for a mutable i32
                let value = value * 10 + node.val; // Calculate current value
                if node.left.is_none() && node.right.is_none() { // Check if the left and right value of TreeNode is None
                    *sum += value; // Added the sum value, use pointer because we're directly add it an not borrowing it or have it's ownership
                }
                Solution::summary(&node.left, value, sum); // Run function with node left, value is copied while sum is just listed since the first call it's already borrowed
                Solution::summary(&node.right, value, sum);// Same as above except it's right
            }
            // If it is None
            None => {}
        }
    }
}
