impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[count - 1] {
                nums[count] = nums[i];
                count += 1;
            }
        }

        return count as i32;
    }
}
