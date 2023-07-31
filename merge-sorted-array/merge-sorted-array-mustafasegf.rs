impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx1 = m as usize;
        let mut idx2 = n as usize;
        let mut idx_merged = (m + n - 1) as usize;

        while idx2 > 0 {
            if idx1 > 0 && nums1[idx1 - 1] > nums2[idx2 - 1] {
                nums1[idx_merged] = nums1[idx1 - 1];
                idx1 -= 1;
            } else {
                nums1[idx_merged] = nums2[idx2 - 1];
                idx2 -= 1;
            }
            idx_merged -= 1;
        }
    }
}
