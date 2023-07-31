use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for n in nums.into_iter() {
            *map.entry(n).or_insert(0) -= 1;
        }

        let mut queue = BinaryHeap::with_capacity(k as usize + 1);

        map.into_iter().for_each(|(key, val)| {
            if queue.len() < k as usize {
                queue.push((val, key));
                return;
            }

            if let Some(&(v, _)) = queue.peek() {
                if val < v {
                    queue.push((val, key));
                }
            }

            if queue.len() > k as usize {
                queue.pop();
            }
        });

        return queue.into_iter().map(|(_, x)| x).collect::<Vec<i32>>();
    }
}
