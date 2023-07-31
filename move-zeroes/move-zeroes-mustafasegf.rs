impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;

        for i in 0..nums.len() {
            match nums[i] {
                0 => {}
                _ => {
                    nums.swap(idx, i);
                    idx += 1;
                }
            }
        }
    }
}
