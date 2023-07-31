use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut lo = 0i32;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut hi = (m * n - 1) as i32;

        while lo <= hi {
            let mid = (lo + (hi - lo) / 2);

            match matrix[mid as usize / n][mid as usize % n].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid - 1,
            }
        }

        return false;
    }
}
