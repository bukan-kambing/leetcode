impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        (1..num_rows).fold(vec![vec![1 as i32]], |mut acc, _| {
            acc.push(
                vec![1]
                    .into_iter()
                    .chain(
                        acc.last()
                            .unwrap()
                            .windows(2)
                            .map(|nums| nums.iter().sum())
                            .chain(vec![1].into_iter()),
                    )
                    .collect::<Vec<i32>>(),
            );
            acc
        })
    }
}
