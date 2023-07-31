impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut idx = 0;

        while (idx <= right) && (right != 0) {
            match nums[idx] {
                0 => {
                    nums.swap(idx, left);
                    left += 1;
                    idx += 1;
                }
                2 => {
                    nums.swap(idx, right);
                    right -= 1;
                }
                _ => idx += 1,
            }
        }
    }
}
