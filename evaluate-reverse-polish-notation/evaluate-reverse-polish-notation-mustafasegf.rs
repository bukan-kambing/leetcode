use std::ops::{Add, Div, Mul, Sub};

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(10_000);

        fn operation(op_fn: fn(i32, i32) -> i32, stack: &mut Vec<i32>) {
            let val2 = stack.pop().unwrap();
            let val1 = stack.pop().unwrap();
            stack.push(op_fn(val1, val2));
        }

        for token in tokens.into_iter() {
            match token.as_str() {
                "+" => operation(i32::add, &mut stack),
                "-" => operation(i32::sub, &mut stack),
                "*" => operation(i32::mul, &mut stack),
                "/" => operation(i32::div, &mut stack),
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }
        return stack.pop().unwrap();
    }
}
