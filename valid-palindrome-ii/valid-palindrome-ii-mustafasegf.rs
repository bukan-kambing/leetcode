impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let arr = s.as_bytes();
        let len = s.len();

        for idx in 0..(len / 2) {
            if arr[idx] != arr[len - idx - 1] {
                let left = &arr[idx..len - idx - 1];
                let right = &arr[idx + 1..len - idx];

                let left_len = left.len();
                let right_len = right.len();

                return (0..(left_len / 2)).all(|idx| left[idx] == left[left_len - idx - 1])
                    || (0..(right_len / 2)).all(|idx| right[idx] == right[right_len - idx - 1]);
            }
        }
        return true;
    }
}
