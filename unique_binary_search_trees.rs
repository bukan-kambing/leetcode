use std::mem::replace;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut num_trees: Vec<i32> = vec![1; (n + 1) as usize];
        for node in 2..n + 1 {
            let mut total = 0;
            for root in 1..node+1 {
                let left = root - 1;
                let right = node - root;
                total += num_trees[left as usize] * num_trees[right as usize];
            }
            replace(&mut num_trees[node as usize], total);
        }
        return num_trees[n as usize];
    }
}
