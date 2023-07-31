use std::cmp::Ordering;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut low = 1;
        let mut high = *piles.iter().max().unwrap();

        while low < high {
            let mid = low + (high - low) / 2;

            match piles.iter().map(|x| (x + mid - 1) / mid).sum::<i32>() > h {
                true => low = mid + 1,
                false => high = mid,
            }
        }

        low
    }
}
