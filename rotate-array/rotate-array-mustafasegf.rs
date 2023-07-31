impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        let k = (k as usize) % n;
        if k == 0 {
            return;
        }

        Self::rev(nums, 0, n - 1);
        Self::rev(nums, 0, k - 1);
        Self::rev(nums, k, n - 1);
    }

    fn rev(nums: &mut Vec<i32>, mut lo: usize, mut hi: usize) {
        while lo < hi {
            nums.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}
