impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let n = prices.len() as i32;
        for i in 1..n {
            if prices[i as usize] > prices[(i - 1) as usize] {
                profit += prices[i as usize] - prices[(i - 1) as usize];
            }
        }
        return profit;
    }
}
