impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        nums.windows(k as usize)
            .map(|students| students[k as usize - 1] - students[0])
            .min()
            .unwrap_or_default()
    }
}
