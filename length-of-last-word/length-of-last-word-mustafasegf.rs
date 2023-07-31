impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .len() as i32
    }
}
