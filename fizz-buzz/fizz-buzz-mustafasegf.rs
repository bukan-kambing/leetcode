impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| {
                let mut temp = String::new();
                if i % 3 == 0 {
                    temp.push_str("Fizz");
                }
                if i % 5 == 0 {
                    temp.push_str("Buzz");
                }
                if temp == "" {
                    temp.push_str(i.to_string().as_str())
                }
                temp
            })
            .collect::<Vec<_>>()
    }
}
