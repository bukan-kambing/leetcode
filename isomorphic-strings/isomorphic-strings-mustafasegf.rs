use std::collections::HashSet;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = Vec::with_capacity(s.len());

        s.chars().zip(t.chars()).all(|(cs, ts)| {
            match map.iter().find(|(k, v)| *k == cs || *v == ts) {
                Some((cs_prev, ts_prev)) => !(ts != *ts_prev || cs != *cs_prev),
                _ => {
                    map.push((cs, ts));
                    true
                }
            }
        })
    }
}
