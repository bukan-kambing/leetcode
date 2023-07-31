use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => map.insert(num, i),
                Some(idx) => return vec![*idx as i32, i as i32],
            };
        }
        return vec![0, 0];
    }
}
