use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::with_capacity(strs.len());

        for s in strs.into_iter() {
            let mut set = [0; 26];
            for c in s.chars() {
                set[(c as u8 - 'a' as u8) as usize] += 1
            }
            map.entry(set).or_insert(Vec::new()).push(s);
        }
        return map.into_values().collect();
    }
}
