impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut same: Vec<i32> = vec![];
        for i in nums.iter() {
            if same.contains(i) {
                if let Some(index) = same.iter().position(|value| *value == i.clone()) {
                    same.swap_remove(index);
                }
            } else {
                same.push(i.clone());
            }
        }
        return same;
    }
}
