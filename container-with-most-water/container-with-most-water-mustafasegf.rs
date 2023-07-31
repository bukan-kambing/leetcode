use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut ans = 0;

        while left < right {
            let area = min(height[left], height[right]) * (right - left) as i32;
            ans = max(area, ans);

            match height[left] <= height[right] {
                true => left += 1,
                false => right -= 1,
            }
        }
        return ans;
    }
}

