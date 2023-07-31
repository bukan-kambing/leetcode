use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k.min(nums.len() as i32 - 1) as usize;

        nums.iter()
            .enumerate()
            .try_fold(HashSet::with_capacity(k), |mut set, (i, num)| {
                if !set.insert(num) {
                    return None;
                }
                if i >= k {
                    set.remove(&nums[i - k]);
                }
                Some(set)
            })
            .is_none()
    }
}
