use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        let mut lo = 0i32;
        let mut hi = n - 1;

        // find pivot
        while lo < hi {
            let mid = (lo + (hi - lo) / 2);
            if nums[lo as usize] < nums[hi as usize] {
                break;
            }

            match nums[lo as usize] <= nums[mid as usize] {
                true => lo = mid + 1,
                false => hi = mid,
            }
        }

        let pivot = lo;
        hi = n - 1;
        lo = 0;

        // adjusted binary search
        while lo <= hi {
            let mid = (lo + (hi - lo) / 2);
            let mid_adjudted = ((lo + (hi - lo) / 2) + pivot) % n;

            match nums[mid_adjudted as usize].cmp(&target) {
                Ordering::Equal => return mid_adjudted,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid - 1,
            }
        }

        return -1;
    }
}
