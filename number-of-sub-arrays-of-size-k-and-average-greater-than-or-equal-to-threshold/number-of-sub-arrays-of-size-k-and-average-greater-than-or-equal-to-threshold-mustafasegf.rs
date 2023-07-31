impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        arr.windows(k as usize)
            .map(|nums| nums.iter().sum::<i32>() / k)
            .filter(|&num| num >= threshold)
            .count() as i32
    }
}
