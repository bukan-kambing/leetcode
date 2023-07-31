impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[lo] < nums[hi] {
                return nums[lo];
            }

            match nums[lo] <= nums[mid] {
                true => lo = mid + 1,
                false => hi = mid,
            }
        }

        nums[lo] as i32
    }
}
