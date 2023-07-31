impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 100_001];

        for num in nums.drain(..) {
            let num = (num + 50_000) as usize;
            counts[num] += 1;
        }

        for num in 0..=100_000 {
            let mut count = counts[num];
            if count > 0 {
                let num = num as i32 - 50_000;
                for _ in 0..count {
                    nums.push(num)
                }
            }
        }
        return nums;
    }
}
