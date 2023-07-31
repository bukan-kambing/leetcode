use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut count = 0;
        let mut left = -1;

        for (i, ch) in s.chars().enumerate() {
            if let Some(idx) = map.insert(ch, i as i32) {
                left = left.max(idx);
            }

            count = count.max(i as i32 - left);
        }
        return count;
    }
}
