impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_idx = 0;
        let mut right_idx = numbers.len() - 1;

        while left_idx < right_idx {
            let left_num = numbers[left_idx];
            let right_num = numbers[right_idx];

            if left_num + right_num == target {
                return vec![left_idx as i32 + 1, right_idx as i32 + 1];
            }

            match right_num + left_num > target {
                true => right_idx -= 1,
                false => left_idx += 1,
            }
        }
        return vec![-1, -1];
    }
}
