use std::cmp::min;
use std::{mem::replace, vec};
impl Solution {
    // Storing value
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let mut profits = vec![0; prices.len()];
    //     let mut minimum = prices[0];
    //     let n = prices.len() as i32;
    //     for i in 1..n {
    //         minimum = min(prices[i as usize], minimum);
    //         replace(&mut profits[i as usize], prices[i as usize] - minimum);
    //     }
    //     return *profits.iter().max().unwrap();
    // }

    // Using pointers
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sell = 0 as usize;
        let mut buy = 0 as usize;
        let mut max_profit = 0;

        while buy < prices.len() {
            if prices[sell] < prices[buy] {
                let profit = prices[buy] - prices[sell];
                max_profit = max(max_profit, profit);
            } else {
                sell = buy;
            }

            buy += 1;
        }

        return max_profit;
    }
}
