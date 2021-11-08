impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 2 {
            return 1 + Solution::climb_stairs(n - 1);
        } else if n == 1 {
            return 1 + Solution::climb_stairs(n - 1);
        } else if n <= 0 {
            return 0;
        }
        return climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
    }
}
