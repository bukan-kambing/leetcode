// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            match self.isBadVersion(mid) {
                true => hi = mid - 1,
                false => lo = mid + 1,
            }
        }

        return hi + 1;
    }
}
