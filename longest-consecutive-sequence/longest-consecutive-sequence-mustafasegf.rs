use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.iter().copied().collect::<HashSet<_>>();

        nums.into_iter()
            .filter(|&num| !set.contains(&(num - 1)))
            .map(|num| (num..).take_while(|x| set.contains(x)).count())
            .max()
            .unwrap_or_default() as i32
    }
}
