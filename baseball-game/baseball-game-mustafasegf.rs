impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = vec![];
        for op in operations {
            match op.as_str() {
                "C" => stack.pop().map(|_| ()).unwrap(),
                "D" => stack.push(stack[stack.len() - 1] * 2),
                "+" => stack.push(stack[stack.len() - 1] + stack[stack.len() - 2]),
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }

        stack.iter().sum()
    }
}
