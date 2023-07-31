impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        s.chars()
            .try_fold(t.chars(), |mut acc, x| acc.find(|&y| x == y).map(|_| acc))
            .is_some()
    }
}
