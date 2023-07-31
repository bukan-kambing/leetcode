impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left = 0;

        return nums
            .iter()
            .enumerate()
            .position(|(i, num)| {
                left += num;
                sum - left == left - num
            })
            .map(|i| i as i32)
            .unwrap_or(-1);
    }
}
