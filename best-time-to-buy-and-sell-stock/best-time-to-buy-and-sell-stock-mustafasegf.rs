impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .into_iter()
            .fold((i32::MAX, 0), |(min, profit), price| match price - min {
                num if num < 0 => (price, profit),
                num if num > profit => (min, num),
                _ => (min, profit),
            })
            .1
    }
}
