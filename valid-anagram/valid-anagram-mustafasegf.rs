impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut set1 = [0; 26];
        let mut set2 = [0; 26];

        for c in s.chars() {
            set1[(c as u8 - 'a' as u8) as usize] += 1;
        }

        for c in t.chars() {
            set2[(c as u8 - 'a' as u8) as usize] += 1;
        }

        return set1 == set2;
    }
}
