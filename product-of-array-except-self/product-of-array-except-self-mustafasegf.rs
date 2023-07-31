impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefix_left = nums
            .iter()
            .scan(1, |acc, &num| {
                let result = *acc;
                *acc *= num;
                Some(result)
            })
            .collect::<Vec<i32>>();

        let prefix_right = nums
            .iter()
            .rev()
            .scan(1, |acc, &num| {
                let result = *acc;
                *acc *= num;
                Some(result)
            })
            .collect::<Vec<i32>>();

        prefix_left
            .iter()
            .zip(prefix_right.iter().rev())
            .map(|(&left, &right)| left * right)
            .collect::<Vec<i32>>()
    }
}
