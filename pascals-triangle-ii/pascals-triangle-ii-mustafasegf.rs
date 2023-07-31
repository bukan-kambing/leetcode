impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as i128;
        (0..=row_index)
            .map(|num| count_combinations(row_index, num))
            .collect()
    }
}

fn factorial(n: i128) -> i128 {
    (1..=n).product()
}

fn count_combinations(n: i128, r: i128) -> i32 {
    ((n - r + 1..=n).product::<i128>() / factorial(r)) as i32
}
