use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0i32;

        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
            if map[&ch] % 2 == 0 {
                ans += 2;
            }
        }

        if s.len() > ans as usize {
            ans += 1;
        }

        ans
    }
}
