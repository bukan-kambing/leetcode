use std::cmp::{max, min};

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }

        let mut left = vec![0; n];
        let mut right = vec![0; n];

        left[0] = height[0];
        right[n - 1] = height[n - 1];

        height.iter().enumerate().skip(1).for_each(|(i, h)| {
            left[i] = max(*h, left[i - 1]);
        });

        height.iter().enumerate().rev().skip(1).for_each(|(i, h)| {
            right[i] = max(*h, right[i + 1]);
        });

        return (0..n).map(|i| min(left[i], right[i]) - height[i]).sum();
    }
}
