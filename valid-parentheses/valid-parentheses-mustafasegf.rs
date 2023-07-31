use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len() / 2);
        let table = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

        for c in s.chars() {
            match table.get(&c) {
                Some(c2) => stack.push(c2),
                None if stack.pop() == Some(&c) => {}
                _ => return false,
            }
        }

        return stack.is_empty();
    }
}
