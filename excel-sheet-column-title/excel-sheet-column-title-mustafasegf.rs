impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut val = column_number - 1;
        let mut ans = Vec::new();

        while val >= 0 {
            ans.push(((val % 26) as u8 + 'A' as u8) as char);
            val = val / 26 - 1;
        }

        ans.iter().rev().collect()
    }
}
