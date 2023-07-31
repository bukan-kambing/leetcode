use std::mem;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut right = -1;
        for num in arr.iter_mut().rev() {
            right = right.max(mem::replace(num, right));
        }
        return arr;
    }
}
