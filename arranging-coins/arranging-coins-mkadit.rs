impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut num = n;
        let mut stair: i32 = 1;
        while num - stair >= 0 {
            num -= stair;
            stair += 1;
        }
        stair - 1
    }
}
