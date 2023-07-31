impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars_iter = s.chars().filter(|c| c.is_ascii_alphanumeric());
        let chars_iter_rev = chars_iter.clone().rev();
        let n = chars_iter.count();

        chars_iter
            .zip(chars_iter_rev)
            .take(n / 2)
            .all(|(c1, c2)| c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
    }
}
