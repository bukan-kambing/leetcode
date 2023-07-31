impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = nums.len().min(2);

        for i in 2..nums.len() {
            if nums[i] != nums[count - 2] {
                nums[count] = nums[i];
                count += 1;
            }
        }

        return count as i32;
    }
}
