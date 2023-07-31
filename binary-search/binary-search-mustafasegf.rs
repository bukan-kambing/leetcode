use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = nums.len() as i32 - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid - 1,
            }
        }

        return -1;
    }
}
